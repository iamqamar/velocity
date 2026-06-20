use std::time::Duration;
use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use gstreamer as gst;
use gstreamer::glib;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use gst::prelude::*;
use gst_video::prelude::*;

use super::frame::VideoFrame;

pub struct Player {
    playbin: gst::Element,
}

impl Player {
    /// Create a new Player instance that sends decoded frames via the channel.
    pub fn new(frame_sender: Sender<VideoFrame>) -> Result<Self> {
        // Initialize GStreamer (safe to call multiple times)
        gst::init().context("failed to initialize GStreamer")?;

        // Create playbin element
        let playbin = gst::ElementFactory::make("playbin")
            .name("velocity-playbin")
            .build()
            .context("failed to create playbin element")?;

        // Create a custom video-sink bin to convert decoders' output to RGBA appsink
        let bin = gst::Bin::new();
        let videoconvert = gst::ElementFactory::make("videoconvert")
            .name("velocity-videoconvert")
            .build()
            .context("failed to create videoconvert element")?;
        let appsink = gst::ElementFactory::make("appsink")
            .name("velocity-appsink")
            .build()
            .context("failed to create appsink element")?;

        let appsink = appsink
            .dynamic_cast::<gst_app::AppSink>()
            .map_err(|_| anyhow::anyhow!("appsink cast failed"))?;

        // Configure appsink
        appsink.set_property("emit-signals", true);
        appsink.set_property("max-buffers", 1u32);
        appsink.set_property("drop", true); // Drop frames if rendering falls behind
        appsink.set_property("sync", true); // Handle A/V sync automatically

        // Set caps for RGBA output
        let caps = gst_video::VideoCapsBuilder::new()
            .format(gst_video::VideoFormat::Rgba)
            .build();
        appsink.set_caps(Some(&caps));

        // Add elements to the custom bin and link them
        bin.add(&videoconvert)?;
        bin.add(&appsink)?;
        videoconvert.link(&appsink)?;

        // Expose the videoconvert's sink pad as a ghost pad of the bin
        let sink_pad = videoconvert.static_pad("sink").context("sink pad not found")?;
        let ghost_pad = gst::GhostPad::with_target(&sink_pad).context("failed to create ghost pad")?;
        bin.add_pad(&ghost_pad)?;

        // Set the custom bin as playbin's video sink
        playbin.set_property("video-sink", &bin);

        // Setup appsink frame callback
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                    let caps = sample.caps().ok_or(gst::FlowError::Error)?;

                    let video_info = gst_video::VideoInfo::from_caps(caps)
                        .map_err(|_| gst::FlowError::Error)?;

                    let frame = gst_video::VideoFrame::from_buffer_readable(
                        buffer.to_owned(), &video_info
                    ).map_err(|_| gst::FlowError::Error)?;

                    let width = frame.width();
                    let height = frame.height();
                    let stride = frame.plane_stride()[0] as usize;
                    let data = frame.plane_data(0).map_err(|_| gst::FlowError::Error)?;

                    // Handle potential stride/padding. Iced expects tightly packed RGBA data (w * h * 4).
                    let packed_data = if stride == width as usize * 4 {
                        data.to_vec()
                    } else {
                        let mut packed = Vec::with_capacity((width * height * 4) as usize);
                        for row in 0..height as usize {
                            let start = row * stride;
                            let end = start + (width as usize * 4);
                            packed.extend_from_slice(&data[start..end]);
                        }
                        packed
                    };

                    let video_frame = VideoFrame {
                        width,
                        height,
                        data: packed_data,
                    };

                    let _ = frame_sender.send(video_frame);

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        Ok(Self { playbin })
    }

    /// Open a file/URL and pause to preroll.
    pub fn open(&self, path_or_uri: &str) -> Result<()> {
        let uri = if path_or_uri.starts_with("http://")
            || path_or_uri.starts_with("https://")
            || path_or_uri.starts_with("file://")
        {
            path_or_uri.to_string()
        } else {
            let path = std::path::Path::new(path_or_uri);
            let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            let gstr: Result<glib::GString, glib::Error> = glib::filename_to_uri(&abs_path, None);
            gstr.map(|u: glib::GString| u.to_string())
                .unwrap_or_else(|_| format!("file:///{}", path_or_uri.replace("\\", "/")))
        };

        // Stop current playback first
        self.stop()?;

        self.playbin.set_property("uri", &uri);

        // Transition to Paused to preroll and load media details
        self.playbin.set_state(gst::State::Paused)?;
        Ok(())
    }

    /// Start playback.
    pub fn play(&self) -> Result<()> {
        self.playbin.set_state(gst::State::Playing)?;
        Ok(())
    }

    /// Pause playback.
    pub fn pause(&self) -> Result<()> {
        self.playbin.set_state(gst::State::Paused)?;
        Ok(())
    }

    /// Toggle play/pause state.
    pub fn toggle(&self) -> Result<()> {
        if self.is_playing() {
            self.pause()
        } else {
            self.play()
        }
    }

    /// Stop playback and release resources.
    pub fn stop(&self) -> Result<()> {
        self.playbin.set_state(gst::State::Null)?;
        Ok(())
    }

    /// Seek to absolute position.
    pub fn seek_to(&self, position: Duration) -> Result<()> {
        let clock_time = gst::ClockTime::from_nseconds(position.as_nanos() as u64);
        self.playbin.seek_simple(
            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
            clock_time,
        ).context("seek failed")?;
        Ok(())
    }

    /// Seek relatively by offset in seconds.
    pub fn seek_relative(&self, offset_secs: i64) -> Result<()> {
        if let Some(pos) = self.position() {
            let new_pos_secs = (pos.as_secs_f64() + offset_secs as f64).max(0.0);
            let new_pos = Duration::from_secs_f64(new_pos_secs);
            self.seek_to(new_pos)?;
        }
        Ok(())
    }

    /// Set volume (0.0 to 1.0).
    pub fn set_volume(&self, volume: f64) -> Result<()> {
        self.playbin.set_property("volume", volume);
        Ok(())
    }

    /// Set mute.
    pub fn set_mute(&self, mute: bool) -> Result<()> {
        self.playbin.set_property("mute", mute);
        Ok(())
    }

    /// Adjust playback speed/rate.
    pub fn set_speed(&self, rate: f64) -> Result<()> {
        // GStreamer changes rate by sending a seek event.
        if let Some(pos) = self.position() {
            let clock_time = gst::ClockTime::from_nseconds(pos.as_nanos() as u64);
            let seek_event = gst::event::Seek::new(
                rate,
                gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                gst::SeekType::Set,
                clock_time,
                gst::SeekType::None,
                gst::ClockTime::NONE,
            );
            self.playbin.send_event(seek_event);
        } else {
            let seek_event = gst::event::Seek::new(
                rate,
                gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                gst::SeekType::Set,
                gst::ClockTime::from_seconds(0),
                gst::SeekType::None,
                gst::ClockTime::NONE,
            );
            self.playbin.send_event(seek_event);
        }
        Ok(())
    }

    /// Query current position.
    pub fn position(&self) -> Option<Duration> {
        self.playbin
            .query_position::<gst::ClockTime>()
            .map(|t| Duration::from_nanos(t.nseconds()))
    }

    /// Query total duration.
    pub fn duration(&self) -> Option<Duration> {
        self.playbin
            .query_duration::<gst::ClockTime>()
            .map(|t| Duration::from_nanos(t.nseconds()))
    }

    /// Check if the player is currently playing.
    pub fn is_playing(&self) -> bool {
        let (res, state, _pending) = self.playbin.state(gst::ClockTime::from_mseconds(10));
        res.is_ok() && state == gst::State::Playing
    }

    /// Get subtitle track count.
    pub fn subtitle_count(&self) -> i32 {
        self.playbin.property::<i32>("n-text")
    }

    /// Select active subtitle track. Set to -1 to disable.
    pub fn set_subtitle_track(&self, index: i32) -> Result<()> {
        self.playbin.set_property("current-text", index);
        Ok(())
    }

    /// Set external subtitle file path/URI.
    pub fn set_external_subtitles(&self, uri: &str) -> Result<()> {
        self.playbin.set_property("suburi", uri);
        Ok(())
    }

    /// Get audio track count.
    pub fn audio_track_count(&self) -> i32 {
        self.playbin.property::<i32>("n-audio")
    }

    /// Select active audio track.
    pub fn set_audio_track(&self, index: i32) -> Result<()> {
        self.playbin.set_property("current-audio", index);
        Ok(())
    }

    /// Returns the bus for monitoring messages.
    pub fn bus(&self) -> Option<gst::Bus> {
        self.playbin.bus()
    }
}
