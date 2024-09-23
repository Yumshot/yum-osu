mod audio;
mod models;
mod visualization;
mod window_config;

use audio::beats::gather_beats;
use macroquad::window::{screen_height, screen_width};
use rodio::{OutputStream, Sink};
use std::time::Instant;
use visualization::pattern::visualize_pattern;
use window_config::window_conf;

#[macroquad::main(window_conf())]
async fn main() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Get window dimensions (you can set these as per your requirement)
    let window_width = 1920.0;
    let window_height = 1080.0;

    // Calculate the position to center the window
    let x = ((screen_width - window_width) / 2.0).floor(); // Cast to u32
    let y = ((screen_height - window_height) / 2.0).floor(); // Cast to u32

    // Set the window position
    macroquad::miniquad::window::set_window_position(x as u32, y as u32); // Cast to i32 for compatibility
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let beats = gather_beats("src/assets/music/hardy.mp3", &sink).await;

    let start_time = Instant::now();
    visualize_pattern(&beats, start_time, &sink).await;
}
