use crate::models::circle::Circle;
use ::rand::Rng;
use macroquad::prelude::*;
use rodio::Sink;
use std::time::Instant;

const SHRINK_TIME: f64 = 1.5;
const CIRCLE_MAX_RADIUS: f32 = 100.0;
const OUTLINE_THICKNESS: f32 = 2.0;
const SCORE_FONT_SIZE: f32 = 40.0;
const DARKGRAY: Color = Color::new(50.0, 50.0, 50.0, 255.0);
const GOLD_COLOR: Color = Color::new(255.0, 223.0, 0.0, 255.0);
const OUTLINE_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.5);
const DRAW_SCORE_X: f32 = 20.0;
const DRAW_SCORE_Y: f32 = 40.0;

pub async fn visualize_pattern(beats: &[f64], start_time: Instant, sink: &Sink) {
    let (width, height) = (screen_width(), screen_height());
    let mut rng = ::rand::thread_rng();

    let spawn_radius = calculate_spawn_radius(width, height);
    let center = Vec2::new(width / 2.0, height / 2.0);

    let mut circles = initialize_circles(beats, &mut rng, spawn_radius, center, SHRINK_TIME);
    let mut score = 0;

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        draw_background(width, height, elapsed);

        handle_key_hits(&mut circles, elapsed, &mut score, SHRINK_TIME);
        draw_circles(&circles, elapsed, SHRINK_TIME);
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
                max_radius: CIRCLE_MAX_RADIUS,
                hit: false,
            }
        })
        .collect()
}

fn handle_key_hits(circles: &mut Vec<Circle>, elapsed: f64, score: &mut i32, shrink_time: f64) {
    let mouse_pos: Vec2 = mouse_position().into();
    let key_pressed = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::S);

    for circle in circles.iter_mut().filter(|c| !c.hit) {
        if let Some(radius) = circle_radius(circle, elapsed, shrink_time) {
            if mouse_pos.distance(circle.position) < radius && key_pressed {
                circle.hit = true;
                *score += calculate_score(circle.hit_time, elapsed);
                break; // Prevent multiple hits in one frame
            }
        }
    }
}

fn circle_radius(circle: &Circle, elapsed: f64, shrink_time: f64) -> Option<f32> {
    let time_since_spawn = elapsed - circle.spawn_time;
    if (0.0..=shrink_time).contains(&time_since_spawn) {
        Some(circle.max_radius * (1.0 - ((time_since_spawn / shrink_time) as f32)))
    } else {
        None
    }
}

fn draw_circles(circles: &Vec<Circle>, elapsed: f64, shrink_time: f64) {
    for circle in circles {
        let time_since_spawn = elapsed - circle.spawn_time;

        if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
            let scale = 1.0 - time_since_spawn / shrink_time;
            let radius = circle.max_radius * (scale as f32);

            // Define the outline color and thickness

            // Draw the outline first (a slightly larger circle behind the main circle)
            draw_circle(
                circle.position.x,
                circle.position.y,
                radius + OUTLINE_THICKNESS,
                OUTLINE_COLOR
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

    // Shadow effect
    draw_text(&score_text, DRAW_SCORE_X + 2.0, DRAW_SCORE_Y + 2.0, SCORE_FONT_SIZE, DARKGRAY);

    // Main score text
    draw_text(&score_text, DRAW_SCORE_X, DRAW_SCORE_Y, SCORE_FONT_SIZE, GOLD_COLOR); // Gold color
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
