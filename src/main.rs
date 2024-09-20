use macroquad::prelude::*;
use rodio::{ Decoder, OutputStream, Sink, Source };
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use ::rand::prelude::*;
use aubio::{ OnsetMode, Onset };

#[derive(Debug)]
struct Circle {
    position: Vec2,
    spawn_time: f64,
    hit_time: f64,
    max_radius: f32,
    hit: bool, // Indicates if the circle has been hit
}

#[macroquad::main("Rhythm Visualizer")]
async fn main() {
    // Initialize audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Gather kick drum beats and start audio playback
    let beats = gather_beats("src/assets/music/hardy.mp3", &sink).await;

    // Start visualization
    let start_time = Instant::now();
    visualize_pattern(&beats, start_time, &sink).await;
}

async fn visualize_pattern(beats: &[f64], start_time: Instant, sink: &Sink) {
    let (width, height) = (screen_width(), screen_height());
    let shrink_time = 1.5;
    let mut rng = thread_rng();

    // Define the maximum spawn radius
    let spawn_radius = width.min(height) / 2.0 - 100.0; // Adjust 100.0 as needed for edge clearance
    let center = Vec2::new(width / 2.0, height / 2.0);

    let mut circles: Vec<Circle> = beats
        .iter()
        .map(|&beat_time| {
            // Generate a random angle and distance within the spawn radius
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(0.0..spawn_radius);

            // Calculate the position using polar coordinates
            let position = Vec2::new(
                center.x + distance * angle.cos(),
                center.y + distance * angle.sin()
            );

            Circle {
                position,
                spawn_time: beat_time - shrink_time,
                hit_time: beat_time,
                max_radius: 100.0,
                hit: false, // Initialize as not hit
            }
        })
        .collect();

    let mut score = 0;

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        clear_background(WHITE);

        // Check for user input (mouse click) to detect a hit
        let mut hit_detected = false; // Track if a circle has been hit this frame
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into(); // Explicit type annotation

            for circle in &mut circles {
                let time_since_spawn = elapsed - circle.spawn_time;

                if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
                    let scale = 1.0 - time_since_spawn / shrink_time;
                    let radius = circle.max_radius * (scale as f32);
                    draw_circle(circle.position.x, circle.position.y, radius, BLUE);

                    // If the click is within the shrinking circle radius, it's a hit
                    let distance = mouse_pos.distance(circle.position);
                    if distance < radius && !hit_detected {
                        circle.hit = true; // Mark the circle as hit
                        score += calculate_score(circle.hit_time, elapsed); // Calculate score based on timing
                        hit_detected = true; // Ensure only one circle is hit per click
                        break; // Stop checking other circles for this click
                    }
                }
            }
        } else {
            // Draw circles normally if no click is detected
            for circle in &mut circles {
                let time_since_spawn = elapsed - circle.spawn_time;
                if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
                    let scale = 1.0 - time_since_spawn / shrink_time;
                    let radius = circle.max_radius * (scale as f32);
                    draw_circle(circle.position.x, circle.position.y, radius, BLUE);
                }
            }
        }

        // Display the score on the screen
        draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, BLACK);

        // Exit loop when audio ends
        if sink.empty() {
            break;
        }

        next_frame().await;
    }
}

// Calculate the score based on the timing of the hit
fn calculate_score(hit_time: f64, current_time: f64) -> i32 {
    let time_difference = (current_time - hit_time).abs();
    if time_difference < 0.1 {
        300 // Perfect hit
    } else if time_difference < 0.3 {
        100 // Good hit
    } else {
        50 // Poor hit
    }
}

async fn gather_beats(path: &str, sink: &Sink) -> Vec<f64> {
    // Decode audio for analysis
    let file = File::open(path).expect("Failed to open audio file");
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader).expect("Failed to decode audio");
    let sample_rate = decoder.sample_rate();

    // Collect samples into a Vec<f32>
    let samples: Vec<f32> = decoder.convert_samples().collect();

    // Detect beats using aubio
    let beats = detect_kick_beats(&samples, sample_rate);

    // Start audio playback
    let file = File::open(path).expect("Failed to open audio file");
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).expect("Failed to decode audio");
    sink.append(source);
    sink.play();

    beats
}

fn detect_kick_beats(samples: &[f32], sample_rate: u32) -> Vec<f64> {
    let buffer_size = 1024;
    let hop_size = 512;
    let mut onset = Onset::new(OnsetMode::Energy, buffer_size, hop_size, sample_rate).unwrap();

    let mut beats = Vec::new();
    let mut buffer = vec![0.0; buffer_size];
    let mut position = 0;

    while position + buffer_size <= samples.len() {
        buffer.copy_from_slice(&samples[position..position + buffer_size]);

        // Process the buffer and check for onsets
        if onset.do_result(&buffer).unwrap() > 0.0 {
            let onset_time = onset.get_last_s();
            beats.push(onset_time as f64);
        }

        position += hop_size;
    }

    beats
}
