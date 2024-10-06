mod audio;
mod models;
mod visualization;
mod window_config;
mod state;

use audio::beats::gather_beats;
use macroquad::{
    color::{ BLACK, LIGHTGRAY, WHITE },
    input::{ is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton },
    shapes::draw_rectangle,
    text::draw_text,
    window::{ next_frame, screen_height, screen_width },
};
use rodio::{ OutputStream, Sink };
use std::time::Instant;
use visualization::pattern::visualize_pattern;
use window_config::window_conf;

#[macroquad::main(window_conf())]
async fn main() {
    let mut state = state::r#enum::GameState::Menu;
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Set the window dimensions and center the window
    let window_width = 1920.0;
    let window_height = 1080.0;
    let x = ((screen_width - window_width) / 2.0).floor();
    let y = ((screen_height - window_height) / 2.0).floor();
    macroquad::miniquad::window::set_window_position(x as u32, y as u32);

    // Rodio audio setup
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop {
        match state {
            state::r#enum::GameState::Menu => {
                // Draw the main menu
                draw_text(
                    "Main Menu",
                    screen_width / 2.0 - 100.0,
                    screen_height / 2.0 - 150.0,
                    40.0,
                    WHITE
                );

                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_position = mouse_position();
                    // Check if the "Start Game" button is clicked
                    if
                        mouse_position.0 > screen_width / 2.0 - 100.0 &&
                        mouse_position.0 < screen_width / 2.0 + 100.0 &&
                        mouse_position.1 > screen_height / 2.0 - 50.0 &&
                        mouse_position.1 < screen_height / 2.0 + 10.0
                    {
                        state = state::r#enum::GameState::Playing;
                    }
                }

                // Render Start Button
                draw_rectangle(
                    screen_width / 2.0 - 100.0,
                    screen_height / 2.0 - 50.0,
                    200.0,
                    60.0,
                    LIGHTGRAY
                );
                draw_text(
                    "Start Game",
                    screen_width / 2.0 - 70.0,
                    screen_height / 2.0 - 10.0,
                    30.0,
                    BLACK
                );
            }

            state::r#enum::GameState::Playing => {
                let beats = gather_beats("src/assets/music/MASHUP.mp3", &sink).await;
                let start_time = Instant::now();
                visualize_pattern(&beats, start_time, &sink).await;

                // Switch back to menu after gameplay (for demo purposes)
                if is_key_pressed(KeyCode::Escape) {
                    state = state::r#enum::GameState::Menu;
                }
            }

            state::r#enum::GameState::End => {
                draw_text(
                    "Game Over",
                    screen_width / 2.0 - 100.0,
                    screen_height / 2.0 - 150.0,
                    40.0,
                    WHITE
                );
            }
        }

        next_frame().await;
    }
}
