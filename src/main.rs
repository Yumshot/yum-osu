mod audio;
mod models;
mod visualization;
mod window_config;

use audio::beats::gather_beats;
use rodio::{ OutputStream, Sink };
use std::time::Instant;
use visualization::pattern::visualize_pattern;
use window_config::window_conf;

#[macroquad::main(window_conf())]
async fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let beats = gather_beats("src/assets/music/hardy.mp3", &sink).await;

    let start_time = Instant::now();
    visualize_pattern(&beats, start_time, &sink).await;
}
