use std::path::PathBuf;
use std::time::{Duration, Instant};
use crossbeam_channel::Receiver;
use iced::widget::{button, column, container, image, row, slider, text, stack};
use iced::{Alignment, Element, Event, Length, Subscription, Task, Theme, window};
use iced::keyboard;
use iced::futures::SinkExt;
use gstreamer as gst;

use crate::engine::{Player, VideoFrame};
use crate::platform::Config;
use crate::utils;

pub struct App {
    pub player: Player,
    pub rx: Receiver<VideoFrame>,
    pub current_frame: Option<VideoFrame>,
    pub position: Duration,
    pub duration: Duration,
    pub volume: f64,
    pub is_muted: bool,
    pub playback_speed: f64,
    pub is_playing: bool,
    pub is_fullscreen: bool,
    pub file_path: Option<PathBuf>,
    pub osd_message: Option<(String, Instant)>,
    pub subtitle_track: i32,
    pub audio_track: i32,
    pub config: Config,
    pub window_id: Option<window::Id>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile(PathBuf),
    FileDropped(PathBuf),
    Play,
    Pause,
    TogglePlay,
    Stop,
    Seek(f64),         // ratio 0.0 - 1.0
    SeekRelative(i64), // seconds (e.g. +5, -5, +30, -30)
    SetVolume(f64),    // 0.0 - 1.0
    VolumeUp,
    VolumeDown,
    ToggleMute,
    SetSpeed(f64),
    SpeedUp,
    SpeedDown,
    SpeedReset,
    NewFrame(VideoFrame),
    PlayerError(String),
    PlayerEos,
    MetaTitle(String),
    Tick,
    ToggleFullscreen,
    CycleSubtitles,
    CycleAudioTrack,
    OsdMessage(String),
    ClearOsd,
    WindowEvent(window::Id, window::Event),
    NoOp,
}

impl App {
    pub fn new(player: Player, rx: Receiver<VideoFrame>) -> Self {
        let config = Config::load();

        // Apply config options to Player
        let _ = player.set_volume(config.volume);
        let _ = player.set_mute(config.muted);
        let _ = player.set_speed(config.playback_speed);

        Self {
            player,
            rx,
            current_frame: None,
            position: Duration::ZERO,
            duration: Duration::ZERO,
            volume: config.volume,
            is_muted: config.muted,
            playback_speed: config.playback_speed,
            is_playing: false,
            is_fullscreen: false,
            file_path: None,
            osd_message: None,
            subtitle_track: -1,
            audio_track: 0,
            config,
            window_id: None,
        }
    }

    pub fn set_osd(&mut self, msg: String) {
        self.osd_message = Some((msg, Instant::now()));
    }
}

pub fn title(state: &App) -> String {
    if let Some(ref path) = state.file_path {
        if let Some(filename) = path.file_name() {
            return format!("Velocity — {}", filename.to_string_lossy());
        }
    }
    "Velocity".to_string()
}

pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::OpenFile(path) => {
            state.file_path = Some(path.clone());
            state.is_playing = false;
            state.position = Duration::ZERO;
            state.duration = Duration::ZERO;
            state.current_frame = None;
            state.subtitle_track = -1;
            state.audio_track = 0;

            state.config.last_directory = path.parent().map(|p| p.to_path_buf());
            let _ = state.config.save();

            if let Err(e) = state.player.open(&path.to_string_lossy()) {
                tracing::error!("Failed to open file: {e}");
                state.set_osd(format!("Error: {}", e));
            } else {
                tracing::info!("Successfully opened media file: {:?}", path);
                state.set_osd(format!("Opened: {}", path.file_name().unwrap_or_default().to_string_lossy()));
                // Play automatically on load
                let _ = state.player.play();
                state.is_playing = true;
            }
        }
        Message::FileDropped(path) => {
            return Task::done(Message::OpenFile(path));
        }
        Message::Play => {
            let _ = state.player.play();
            state.is_playing = true;
            state.set_osd("Play".to_string());
        }
        Message::Pause => {
            let _ = state.player.pause();
            state.is_playing = false;
            state.set_osd("Pause".to_string());
        }
        Message::TogglePlay => {
            let _ = state.player.toggle();
            state.is_playing = state.player.is_playing();
            if state.is_playing {
                state.set_osd("Play".to_string());
            } else {
                state.set_osd("Pause".to_string());
            }
        }
        Message::Stop => {
            let _ = state.player.stop();
            state.is_playing = false;
            state.position = Duration::ZERO;
            state.current_frame = None;
            state.set_osd("Stop".to_string());
        }
        Message::Seek(ratio) => {
            let target_secs = ratio * state.duration.as_secs_f64();
            let target = Duration::from_secs_f64(target_secs);
            let _ = state.player.seek_to(target);
            state.position = target;
        }
        Message::SeekRelative(offset) => {
            let _ = state.player.seek_relative(offset);
            if offset > 0 {
                state.set_osd(format!("+{}s", offset));
            } else {
                state.set_osd(format!("{}s", offset));
            }
        }
        Message::SetVolume(vol) => {
            state.volume = vol.clamp(0.0, 1.0);
            let _ = state.player.set_volume(state.volume);
            state.is_muted = false;
            let _ = state.player.set_mute(false);

            state.config.volume = state.volume;
            state.config.muted = false;
            let _ = state.config.save();

            state.set_osd(format!("Volume: {}%", (state.volume * 100.0).round()));
        }
        Message::VolumeUp => {
            let new_vol = (state.volume + 0.05).min(1.0);
            return Task::done(Message::SetVolume(new_vol));
        }
        Message::VolumeDown => {
            let new_vol = (state.volume - 0.05).max(0.0);
            return Task::done(Message::SetVolume(new_vol));
        }
        Message::ToggleMute => {
            state.is_muted = !state.is_muted;
            let _ = state.player.set_mute(state.is_muted);

            state.config.muted = state.is_muted;
            let _ = state.config.save();

            if state.is_muted {
                state.set_osd("Muted".to_string());
            } else {
                state.set_osd(format!("Volume: {}%", (state.volume * 100.0).round()));
            }
        }
        Message::SetSpeed(speed) => {
            state.playback_speed = speed.clamp(0.25, 4.0);
            let _ = state.player.set_speed(state.playback_speed);

            state.config.playback_speed = state.playback_speed;
            let _ = state.config.save();

            state.set_osd(format!("Speed: {:.2}x", state.playback_speed));
        }
        Message::SpeedUp => {
            let new_speed = (state.playback_speed + 0.1).min(4.0);
            return Task::done(Message::SetSpeed(new_speed));
        }
        Message::SpeedDown => {
            let new_speed = (state.playback_speed - 0.1).max(0.25);
            return Task::done(Message::SetSpeed(new_speed));
        }
        Message::SpeedReset => {
            return Task::done(Message::SetSpeed(1.0));
        }
        Message::NewFrame(frame) => {
            if state.current_frame.is_none() {
                tracing::info!("First video frame decoded successfully: {}x{}", frame.width, frame.height);
            }
            state.current_frame = Some(frame);
        }
        Message::PlayerError(err) => {
            tracing::error!("GStreamer pipeline error: {err}");
            state.set_osd(format!("Error: {}", err));
            state.is_playing = false;
        }
        Message::PlayerEos => {
            tracing::info!("Playback reached End-Of-Stream");
            let _ = state.player.stop();
            state.is_playing = false;
            state.position = Duration::ZERO;
            state.current_frame = None;
            state.set_osd("Playback Finished".to_string());
        }
        Message::MetaTitle(_title) => {
            // Can be used to change window title
        }
        Message::Tick => {
            if state.is_playing {
                if let Some(pos) = state.player.position() {
                    state.position = pos;
                }
                if let Some(dur) = state.player.duration() {
                    state.duration = dur;
                }
            }
            // Clear expired OSD messages
            if let Some((_, time)) = state.osd_message {
                if time.elapsed() >= Duration::from_secs(2) {
                    state.osd_message = None;
                }
            }
        }
        Message::ToggleFullscreen => {
            if let Some(id) = state.window_id {
                state.is_fullscreen = !state.is_fullscreen;
                let mode = if state.is_fullscreen {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                return window::change_mode(id, mode);
            }
        }
        Message::CycleSubtitles => {
            let count = state.player.subtitle_count();
            if count <= 0 {
                state.set_osd("No Subtitles".to_string());
            } else {
                let next = if state.subtitle_track >= count - 1 {
                    -1
                } else {
                    state.subtitle_track + 1
                };
                let _ = state.player.set_subtitle_track(next);
                state.subtitle_track = next;
                if next == -1 {
                    state.set_osd("Subtitles: Off".to_string());
                } else {
                    state.set_osd(format!("Subtitles: Track {}/{}", next + 1, count));
                }
            }
        }
        Message::CycleAudioTrack => {
            let count = state.player.audio_track_count();
            if count <= 1 {
                state.set_osd("Single Audio Track".to_string());
            } else {
                let next = if state.audio_track >= count - 1 {
                    0
                } else {
                    state.audio_track + 1
                };
                let _ = state.player.set_audio_track(next);
                state.audio_track = next;
                state.set_osd(format!("Audio Track: {}/{}", next + 1, count));
            }
        }
        Message::OsdMessage(msg) => {
            state.set_osd(msg);
        }
        Message::ClearOsd => {
            state.osd_message = None;
        }
        Message::WindowEvent(id, event) => {
            state.window_id = Some(id);
            match event {
                window::Event::FileDropped(path) => {
                    return Task::done(Message::OpenFile(path));
                }
                _ => {}
            }
        }
        Message::NoOp => {}
    }
    Task::none()
}

pub fn subscription(state: &App) -> Subscription<Message> {
    let subs = vec![
        // Decoded video frame channel receiver
        Subscription::run_with_id(
            "video-frames",
            iced::stream::channel(10, {
                let rx = state.rx.clone();
                move |mut output| async move {
                    loop {
                        let rx_clone = rx.clone();
                        let frame = tokio::task::spawn_blocking(move || rx_clone.recv().ok()).await;
                        if let Ok(Some(f)) = frame {
                            let _ = output.send(Message::NewFrame(f)).await;
                        } else {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                        }
                    }
                }
            })
        ),

        // GStreamer bus monitor
        Subscription::run_with_id(
            "gstreamer-bus",
            iced::stream::channel(10, {
                let bus = state.player.bus();
                move |mut output| async move {
                    if let Some(b) = bus {
                        loop {
                            let b_clone = b.clone();
                            let msg = tokio::task::spawn_blocking(move || {
                                b_clone.timed_pop(gst::ClockTime::from_mseconds(100))
                            })
                            .await;

                            if let Ok(Some(msg)) = msg {
                                use gst::MessageView;
                                let out_msg = match msg.view() {
                                    MessageView::Error(err) => {
                                        Message::PlayerError(format!("{}", err.error()))
                                    }
                                    MessageView::Eos(_) => {
                                        Message::PlayerEos
                                    }
                                    MessageView::Tag(tag_msg) => {
                                        let tags = tag_msg.tags();
                                        if let Some(title) = tags.get::<gst::tags::Title>() {
                                            Message::MetaTitle(title.get().to_string())
                                        } else {
                                            Message::NoOp
                                        }
                                    }
                                    _ => Message::NoOp,
                                };
                                if !matches!(out_msg, Message::NoOp) {
                                    let _ = output.send(out_msg).await;
                                }
                            }
                        }
                    } else {
                        tokio::time::sleep(Duration::from_secs(3600)).await;
                    }
                }
            })
        ),

        // Timer to query duration/position
        iced::time::every(Duration::from_millis(200)).map(|_| Message::Tick),

        // Window events (captures window ID and handles file drops)
        iced::window::events().map(|(id, event)| Message::WindowEvent(id, event)),

        // Keyboard commands
        iced::event::listen().map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                match key {
                    keyboard::Key::Named(keyboard::key::Named::Space) => Message::TogglePlay,
                    keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => Message::SeekRelative(-5),
                    keyboard::Key::Named(keyboard::key::Named::ArrowRight) => Message::SeekRelative(5),
                    keyboard::Key::Named(keyboard::key::Named::Backspace) => Message::SpeedReset,
                    keyboard::Key::Character(s) => {
                        match s.as_str() {
                            "f" | "F" => Message::ToggleFullscreen,
                            "m" | "M" => Message::ToggleMute,
                            "[" => Message::SpeedDown,
                            "]" => Message::SpeedUp,
                            "v" | "V" => Message::CycleSubtitles,
                            "b" | "B" => Message::CycleAudioTrack,
                            _ => Message::NoOp,
                        }
                    }
                    _ => Message::NoOp,
                }
            }
            _ => Message::NoOp,
        }),
    ];

    Subscription::batch(subs)
}

pub fn view(state: &App) -> Element<'_, Message> {
    // 1. Video viewport or center splash screen
    let mut viewport: Element<Message> = if let Some(frame) = &state.current_frame {
        let handle = image::Handle::from_rgba(frame.width, frame.height, frame.data.clone());
        image(handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Contain)
            .into()
    } else {
        container(
            column![
                text("Velocity")
                    .size(64)
                    .font(iced::Font::DEFAULT),
                text("Drag & Drop a video file here to start playing")
                    .size(18)
                    .style(text::secondary),
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    };

    // 2. Overlay OSD on top of viewport if active
    if let Some((msg, _)) = &state.osd_message {
        let osd_box = container(
            text(msg)
                .size(20)
                .color(iced::Color::WHITE)
                .font(iced::Font::MONOSPACE),
        )
        .padding(12)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.75).into()),
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

        viewport = stack![
            viewport,
            container(osd_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        ]
        .into();
    }

    // 3. Build controls bar
    let play_pause_btn = button(text(if state.is_playing { "⏸" } else { "▶" }))
        .on_press(Message::TogglePlay)
        .style(button::text);

    let stop_btn = button(text("⏹"))
        .on_press(Message::Stop)
        .style(button::text);

    let current_secs = state.position.as_secs();
    let total_secs = state.duration.as_secs();
    let time_str = format!(
        "{} / {}",
        utils::time::format_duration(current_secs),
        utils::time::format_duration(total_secs)
    );
    let time_display = text(time_str)
        .size(14)
        .font(iced::Font::MONOSPACE);

    let seek_ratio = if total_secs > 0 {
        state.position.as_secs_f64() / state.duration.as_secs_f64()
    } else {
        0.0
    };

    // Nice seekbar
    let seekbar = slider(0.0..=1.0, seek_ratio, Message::Seek)
        .step(0.001);

    let vol_icon = if state.is_muted || state.volume == 0.0 {
        "🔇"
    } else if state.volume < 0.3 {
        "🔈"
    } else if state.volume < 0.7 {
        "🔉"
    } else {
        "🔊"
    };

    let mute_btn = button(text(vol_icon))
        .on_press(Message::ToggleMute)
        .style(button::text);

    let volume_slider = slider(0.0..=1.0, state.volume, Message::SetVolume)
        .step(0.01)
        .width(80);

    let speed_btn = button(text(format!("{:.2}x", state.playback_speed)))
        .on_press(Message::SpeedReset)
        .style(button::text);

    let sub_btn = button(text("💬"))
        .on_press(Message::CycleSubtitles)
        .style(button::text);

    let audio_btn = button(text("🎵"))
        .on_press(Message::CycleAudioTrack)
        .style(button::text);

    let fullscreen_btn = button(text(if state.is_fullscreen { "⛶" } else { "fullscreen" }))
        .on_press(Message::ToggleFullscreen)
        .style(button::text);

    let controls_bar = container(
        column![
            seekbar,
            row![
                play_pause_btn,
                stop_btn,
                time_display,
                row![].width(Length::Fill), // spacer
                audio_btn,
                sub_btn,
                speed_btn,
                mute_btn,
                volume_slider,
                fullscreen_btn,
            ]
            .spacing(15)
            .align_y(Alignment::Center)
        ]
        .spacing(5),
    )
    .padding(10)
    .style(|_theme: &Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb(0.08, 0.08, 0.10).into()),
            ..Default::default()
        }
    });

    column![
        viewport,
        controls_bar,
    ]
    .into()
}
