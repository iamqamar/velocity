use std::path::PathBuf;
use clap::Parser;
use crossbeam_channel::unbounded;
use iced::{window, Size, Task};

pub mod app;
pub mod engine;
pub mod platform;
pub mod ui;
pub mod utils;

use app::{App, Message};
use engine::Player;

#[derive(Parser, Debug)]
#[command(
    name = "Velocity",
    version,
    about = "A modern, high-performance video player",
    long_about = None
)]
struct Args {
    /// Path to a video file to play
    file: Option<PathBuf>,
}

fn main() -> iced::Result {
    // Initialize logging
    utils::logger::init();

    // Parse command line arguments
    let args = Args::parse();
    let file_to_open = args.file;

    // Launch iced application using functional builder
    iced::application(app::title, app::update, app::view)
        .theme(|_| ui::theme::velocity_dark())
        .subscription(app::subscription)
        .window(window::Settings {
            size: Size::new(1280.0, 720.0),
            ..Default::default()
        })
        .run_with(move || {
            let (tx, rx) = unbounded();

            // Create player
            let player = match Player::new(tx) {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to initialize GStreamer: {e}");
                    panic!("Failed to initialize GStreamer: {e}");
                }
            };

            let app_state = App::new(player, rx);

            // If a file was passed as argument, open it immediately
            let task = if let Some(path) = file_to_open {
                Task::done(Message::OpenFile(path))
            } else {
                Task::none()
            };

            (app_state, task)
        })
}
