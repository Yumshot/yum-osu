use crate::structs::{ Circle, FloatingText };
use crate::constants::*;
use macroquad::prelude::{ Vec2, KeyCode, mouse_position, is_key_pressed };
use rand::Rng;

/// Initialize circles for a game. Each circle is given a random position around the `center` point
/// within `spawn_radius` and a random angle. The spawn time is the given `beat_time` minus `shrink_time`
/// plus `delay`. The hit time is the given `beat_time` plus `delay`.
pub fn initialize_circles(
    beats: &[f64],
    rng: &mut impl Rng,
    spawn_radius: f32,
    center: Vec2,
    shrink_time: f64,
    delay: f64
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
                spawn_time: beat_time - shrink_time + delay,
                hit_time: beat_time + delay,
                max_radius: CIRCLE_MAX_RADIUS,
                hit: false,
                missed: false,
            }
        })
        .collect()
}

/// Check if a circle is hit by the player. If the player has pressed the key and the mouse is within
/// the circle's radius, mark the circle as hit and increase the score. The score is increased by
/// `calculate_score(hit_time, elapsed)`, which is a function that takes the hit time and the current time
/// and returns the score.
pub fn handle_key_hits(circles: &mut Vec<Circle>, elapsed: f64, score: &mut i32, shrink_time: f64) {
    let mouse_pos: Vec2 = mouse_position().into();
    let key_pressed = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::S);

    for circle in circles.iter_mut().filter(|c| !c.hit) {
        if let Some(radius) = circle_radius(circle, elapsed, shrink_time) {
            if mouse_pos.distance(circle.position) < radius && key_pressed {
                circle.hit = true;
                *score += calculate_score(circle.hit_time, elapsed);
                break;
            }
        }
    }
}

/// Calculate the current radius of a circle given the elapsed time and the shrink time. If the circle
/// is not yet spawned, returns None.
fn circle_radius(circle: &Circle, elapsed: f64, shrink_time: f64) -> Option<f32> {
    let time_since_spawn = elapsed - circle.spawn_time;
    if (0.0..=shrink_time).contains(&time_since_spawn) {
        Some(circle.max_radius * (1.0 - ((time_since_spawn / shrink_time) as f32)))
    } else {
        None
    }
}

/// Calculate the initial spawn radius of the circles based on the screen width and height.
pub fn calculate_spawn_radius(width: f32, height: f32) -> f32 {
    width.min(height) / 2.0 - 100.0
}

/// Check if a circle is missed by the player. If a circle is not yet hit and the elapsed time is greater
/// than the shrink time, mark the circle as missed and add a "Miss" text to the floating texts.
pub fn handle_missed_circles(
    circles: &mut Vec<Circle>,
    elapsed: f64,
    floating_texts: &mut Vec<FloatingText>,
    shrink_time: f64
) {
    for circle in circles.iter_mut().filter(|c| !c.hit && !c.missed) {
        let time_since_spawn = elapsed - circle.spawn_time;

        if time_since_spawn > shrink_time {
            circle.missed = true;

            floating_texts.push(FloatingText {
                text: "Miss".to_string(),
                position: circle.position,
                spawn_time: elapsed,
                duration: 1.0,
            });
        }
    }
}

/// Calculate the score based on the difference between the hit time and the current time. The score is
/// higher if the difference is smaller.
pub fn calculate_score(hit_time: f64, current_time: f64) -> i32 {
    let time_difference = (current_time - hit_time).abs();
    if time_difference < 0.1 {
        300
    } else if time_difference < 0.3 {
        100
    } else {
        50
    }
}

/// Draw all the circles in the given vector. The radius of the circle is calculated based on the elapsed
/// time and the shrink time. The color of the circle changes over time.
pub fn draw_circles(circles: &Vec<Circle>, elapsed: f64, shrink_time: f64) {
    use macroquad::prelude::{ draw_circle, Color };
    for circle in circles {
        let time_since_spawn = elapsed - circle.spawn_time;

        if (0.0..=shrink_time).contains(&time_since_spawn) && !circle.hit {
            let scale = 1.0 - time_since_spawn / shrink_time;
            let radius = circle.max_radius * (scale as f32);

            draw_circle(
                circle.position.x,
                circle.position.y,
                radius + OUTLINE_THICKNESS,
                OUTLINE_COLOR
            );

            let color = Color::new(
                0.2 + (scale as f32) * 0.8,
                0.4,
                0.8 - (scale as f32) * 0.8,
                0.8 - (scale as f32) * 0.5
            );

            draw_circle(circle.position.x, circle.position.y, radius, color);
        }
    }
}
