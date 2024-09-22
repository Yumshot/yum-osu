use crate::models::circle::Circle;
use ::rand::Rng;
use macroquad::prelude::*;
use rodio::Sink;
use std::time::Instant;

pub async fn visualize_pattern(beats: &[f64], start_time: Instant, sink: &Sink) {
    let (width, height) = (screen_width(), screen_height());
    let mut rng = ::rand::thread_rng();
    let shrink_time = 1.5;

    let spawn_radius = calculate_spawn_radius(width, height);
    let center = Vec2::new(width / 2.0, height / 2.0);

    let mut circles = initialize_circles(beats, &mut rng, spawn_radius, center, shrink_time);
    let mut score = 0;

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        draw_background(width, height, elapsed);

        handle_key_hits(&mut circles, elapsed, &mut score, shrink_time);
        draw_circles(&circles, elapsed, shrink_time);
        draw_score(score);

        if sink.empty() {
            break;
        }

        next_frame().await;
    }
}

fn calculate_spawn_radius(width: f32, height: f32) -> f32 {
    width.min(height) / 2.0 - 100.0
}

fn initialize_circles(
    beats: &[f64],
    rng: &mut impl Rng,
    spawn_radius: f32,
    center: Vec2,
    shrink_time: f64
) -> Vec<Circle> {
    beats
        .iter()
        .map(|&beat_time| {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(0.0..spawn_radius);

            let position = Vec2::new(
                center.x + distance * angle.cos(),
                center.y + distance * angle.sin()
            );

            Circle {
                position,
                spawn_time: beat_time - shrink_time,
                hit_time: beat_time,
                max_radius: 100.0,
                hit: false,
            }
        })
        .collect()
}

fn handle_key_hits(circles: &mut Vec<Circle>, elapsed: f64, score: &mut i32, shrink_time: f64) {
    let mouse_pos: Vec2 = mouse_position().into();
    let mut hit_detected = false;

    // Check if A or S keys are pressed
    let a_pressed = is_key_pressed(KeyCode::A);
    let s_pressed = is_key_pressed(KeyCode::S);

    // Iterate through the circles to check for hits based on mouse position and key presses
    for circle in circles.iter_mut() {
        let time_since_spawn = elapsed - circle.spawn_time;

        if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
            let scale = 1.0 - time_since_spawn / shrink_time;
            let radius = circle.max_radius * (scale as f32);

            // Calculate the distance from the mouse position to the circle position
            let distance = mouse_pos.distance(circle.position);

            // Hit detection: the target is hit if the mouse is within the circle's radius
            // and one of the specified keys is pressed
            if distance < radius && !hit_detected && (a_pressed || s_pressed) {
                circle.hit = true;
                *score += calculate_score(circle.hit_time, elapsed);
                hit_detected = true; // Prevent further hits in this frame
            }
        }
    }
}

fn draw_circles(circles: &Vec<Circle>, elapsed: f64, shrink_time: f64) {
    for circle in circles {
        let time_since_spawn = elapsed - circle.spawn_time;

        if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
            let scale = 1.0 - time_since_spawn / shrink_time;
            let radius = circle.max_radius * (scale as f32);

            // Define the outline color and thickness
            let outline_color = Color::new(0.0, 0.0, 0.0, 0.5); // Semi-transparent black
            let outline_thickness = 2.0; // Adjust thickness as needed

            // Draw the outline first (a slightly larger circle behind the main circle)
            draw_circle(
                circle.position.x,
                circle.position.y,
                radius + outline_thickness,
                outline_color
            );

            // Change circle color based on time since spawn
            let color = Color::new(
                0.2 + (scale as f32) * 0.8, // Red component varies
                0.4,
                0.8 - (scale as f32) * 0.8, // Blue component varies
                0.8 - (scale as f32) * 0.5
            );

            // Draw the main circle
            draw_circle(circle.position.x, circle.position.y, radius, color);
        }
    }
}

fn draw_score(score: i32) {
    // Styled score display with shadow
    let score_text = format!("Score: {}", score);
    let x = 20.0;
    let y = 40.0;

    // Shadow effect
    draw_text(&score_text, x + 2.0, y + 2.0, 40.0, DARKGRAY);

    // Main score text
    draw_text(&score_text, x, y, 40.0, Color::from_rgba(255, 223, 0, 255)); // Gold color
}

fn draw_background(width: f32, height: f32, elapsed: f64) {
    // Gradient background with slight animation
    let color1 = Color::new(0.1, 0.1, 0.3, 1.0);
    let color2 = Color::new(0.2, 0.2, 0.5, 1.0);
    let offset = (elapsed.sin() as f32) * 0.1;

    // Top to bottom gradient
    for y in 0..height as i32 {
        let t = (y as f32) / height + offset;
        let blend_color = Color::new(
            color1.r * (1.0 - t) + color2.r * t,
            color1.g * (1.0 - t) + color2.g * t,
            color1.b * (1.0 - t) + color2.b * t,
            1.0
        );
        draw_line(0.0, y as f32, width, y as f32, 1.0, blend_color);
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
