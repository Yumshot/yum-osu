mod constants;
mod structs;
mod audio;
mod ui;
mod game;

use crate::structs::*;
use crate::constants::*;
use crate::audio::*;
use crate::ui::*;
use crate::game::*;

use macroquad::prelude::*;
use rodio::{ Decoder, OutputStream, Sink };
use std::{ sync::mpsc, thread, time::Instant };

// Main game loop
#[macroquad::main(window_conf)]
async fn main() {
    // Start in the menu state
    let mut state = GameState::Menu;

    // Keep track of the selected song
    let mut selected_song = String::new();

    // Load songs from assets
    let mut songs = Vec::new();

    // Set up the audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load UI assets
    let assets = load_ui_assets().await;

    // Main game loop
    loop {
        match state {
            // Menu state
            GameState::Menu => {
                // Draw the menu
                if draw_menu(&assets) {
                    // Load songs from assets
                    songs = load_songs_from_assets();

                    // Switch to the song selection state
                    state = GameState::SongSelection;
                }
            }

            // Song selection state
            GameState::SongSelection => {
                // Initialize the song selection state
                let mut selection_state = SongSelectionState::new();

                // Draw the song selection UI
                if let Some(song) = draw_choose_audio(&mut selection_state, &songs) {
                    // Set the selected song
                    selected_song = song;

                    // Switch to the playing state
                    state = GameState::Playing;
                }
            }

            // Playing state
            GameState::Playing => {
                // Start the beat detection in a new thread
                let (tx, rx) = mpsc::channel();
                let song_path = selected_song.clone();
                thread::spawn(move || {
                    // Load the audio file and detect the beats
                    let beats = gather_beats(&song_path);
                    // Send the beats back to the main thread
                    tx.send(beats).unwrap();
                });

                // Switch to the loading state
                state = GameState::Loading {
                    // Store the channel to receive the beats
                    rx,
                    // Store the start time of the loading state
                    start_time: Instant::now(),
                };
            }

            // Loading state
            GameState::Loading { ref rx, ref start_time } => {
                // Display the loading bar
                let loading_time = start_time.elapsed().as_secs_f32();
                draw_loading_bar(loading_time);

                // Check if the beats are received
                if let Ok(beats) = rx.try_recv() {
                    // Load the audio file but don't play it yet
                    let file = std::fs::File
                        ::open(&selected_song)
                        .expect("Failed to open audio file");
                    let reader = std::io::BufReader::new(file);
                    let source = Decoder::new(reader).expect("Failed to decode audio");

                    // Switch to the ready to play state
                    state = GameState::ReadyToPlay {
                        // Store the beats
                        beats,
                        // Store the start time of the ready to play state
                        ready_time: Instant::now(),
                        // Store the audio source
                        source: Some(source),
                    };
                }
            }

            // Ready to play state
            GameState::ReadyToPlay { ref beats, ref ready_time, ref mut source } => {
                // Display the countdown
                let elapsed = ready_time.elapsed().as_secs_f32();
                if elapsed < (COUNTDOWN_DURATION as f32) {
                    let scr_width = screen_width();
                    let scr_height = screen_height();

                    draw_text(
                        &format!("Starting in {:.0}", COUNTDOWN_DURATION - (elapsed as f64)),
                        scr_width / 2.0 - 100.0,
                        scr_height / 2.0,
                        40.0,
                        WHITE
                    );
                } else {
                    // Start the audio playback
                    if let Some(source) = source.take() {
                        sink.append(source);
                        sink.play();
                    }

                    // Initialize the visualization state
                    let (width, height) = (screen_width(), screen_height());
                    let mut rng = ::rand::thread_rng();

                    let spawn_radius = calculate_spawn_radius(width, height);
                    let center = Vec2::new(width / 2.0, height / 2.0);

                    let circles = initialize_circles(
                        beats,
                        &mut rng,
                        spawn_radius,
                        center,
                        SHRINK_TIME,
                        COUNTDOWN_DURATION // Pass the delay here
                    );
                    let score = 0;
                    let floating_texts = Vec::new();

                    // Switch to the visualizing state
                    state = GameState::Visualizing(
                        Box::new(VisualizingState {
                            // Store the beats
                            beats: beats.clone(),
                            // Store the start time of the visualizing state
                            start_time: Instant::now(),
                            circles,
                            score,
                            floating_texts,
                        })
                    );
                }
            }

            // Visualizing state
            GameState::Visualizing(ref mut vis_state) => {
                // Draw the background
                let elapsed = vis_state.start_time.elapsed().as_secs_f64();
                let (width, height) = (screen_width(), screen_height());

                draw_background(width, height, elapsed);

                // Handle inputs, update circles, draw circles, etc.
                handle_key_hits(&mut vis_state.circles, elapsed, &mut vis_state.score, SHRINK_TIME);
                handle_missed_circles(
                    &mut vis_state.circles,
                    elapsed,
                    &mut vis_state.floating_texts,
                    SHRINK_TIME
                );
                draw_circles(&vis_state.circles, elapsed, SHRINK_TIME);
                draw_floating_texts(&mut vis_state.floating_texts, elapsed);
                draw_score(vis_state.score);

                // Check if the user wants to quit
                if is_key_pressed(KeyCode::Escape) {
                    // Optionally stop the music
                    sink.stop();
                    state = GameState::Menu;
                }

                // Check if the music has ended
                if sink.empty() {
                    // Switch to the end state
                    state = GameState::End;
                }
            }

            // End state
            GameState::End => {
                // Clear the screen
                clear_background(BLACK);

                // Draw the game over text
                let scr_width = screen_width();
                let scr_height = screen_height();

                draw_text(
                    "Game Over",
                    scr_width / 2.0 - 100.0,
                    scr_height / 2.0 - 150.0,
                    40.0,
                    WHITE
                );
                draw_text(
                    "Press any key to return to the main menu.",
                    scr_width / 2.0 - 200.0,
                    scr_height / 2.0 - 100.0,
                    30.0,
                    WHITE
                );

                // Check if the user wants to quit
                if is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) {
                    state = GameState::Menu;
                }
            }
        }

        // Wait for the next frame
        next_frame().await;
    }
}
