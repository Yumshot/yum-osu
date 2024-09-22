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
        clear_background(WHITE);

        handle_mouse_click(&mut circles, elapsed, &mut score, shrink_time);
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

fn handle_mouse_click(circles: &mut Vec<Circle>, elapsed: f64, score: &mut i32, shrink_time: f64) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos: Vec2 = mouse_position().into();
        let mut hit_detected = false;

        for circle in circles.iter_mut() {
            let time_since_spawn = elapsed - circle.spawn_time;

            if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
                let scale = 1.0 - time_since_spawn / shrink_time;
                let radius = circle.max_radius * (scale as f32);

                let distance = mouse_pos.distance(circle.position);
                if distance < radius && !hit_detected {
                    circle.hit = true;
                    *score += calculate_score(circle.hit_time, elapsed);
                    hit_detected = true;
                }
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
            draw_circle(circle.position.x, circle.position.y, radius, BLUE);
        }
    }
}

fn draw_score(score: i32) {
    draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, BLACK);
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
