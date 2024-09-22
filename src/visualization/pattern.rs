use crate::models::circle::Circle;
use ::rand::Rng;
use macroquad::prelude::*;
use rodio::Sink;
use std::time::Instant;

pub async fn visualize_pattern(beats: &[f64], start_time: Instant, sink: &Sink) {
    let (width, height) = (screen_width(), screen_height());
    let shrink_time = 1.5;
    let mut rng = ::rand::thread_rng();

    let spawn_radius = width.min(height) / 2.0 - 100.0;
    let center = Vec2::new(width / 2.0, height / 2.0);

    let mut circles: Vec<Circle> = beats
        .iter()
        .map(|&beat_time| {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(0.0..spawn_radius);

            let position = Vec2::new(
                center.x + distance * angle.cos(),
                center.y + distance * angle.sin(),
            );

            Circle {
                position,
                spawn_time: beat_time - shrink_time,
                hit_time: beat_time,
                max_radius: 100.0,
                hit: false,
            }
        })
        .collect();

    let mut score = 0;

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        clear_background(WHITE);

        // Check for user input (mouse click) to detect a hit
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();

            // Track if any circle has been hit during this frame
            let mut hit_detected = false;

            for circle in &mut circles {
                let time_since_spawn = elapsed - circle.spawn_time;

                if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
                    let scale = 1.0 - time_since_spawn / shrink_time;
                    let radius = circle.max_radius * (scale as f32);
                    draw_circle(circle.position.x, circle.position.y, radius, BLUE);

                    // Check if the mouse click is within the shrinking circle's radius
                    let distance = mouse_pos.distance(circle.position);
                    if distance < radius && !hit_detected {
                        circle.hit = true;
                        score += calculate_score(circle.hit_time, elapsed);
                        hit_detected = true; // Prevent further hits in this frame
                    }
                }
            }
        }

        // Draw all circles that are not hit or are still within the shrink time
        for circle in &circles {
            let time_since_spawn = elapsed - circle.spawn_time;
            if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
                let scale = 1.0 - time_since_spawn / shrink_time;
                let radius = circle.max_radius * (scale as f32);
                draw_circle(circle.position.x, circle.position.y, radius, BLUE);
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

fn calculate_score(hit_time: f64, current_time: f64) -> i32 {
    let time_difference = (current_time - hit_time).abs();
    if time_difference < 0.1 {
        300
    } else if time_difference < 0.3 {
        100
    } else {
        50
    }
}
