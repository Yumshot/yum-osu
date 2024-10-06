// src/constants.rs

use macroquad::prelude::*;

pub const SHRINK_TIME: f64 = 1.5;
pub const CIRCLE_MAX_RADIUS: f32 = 100.0;
pub const OUTLINE_THICKNESS: f32 = 2.0;
pub const SCORE_FONT_SIZE: f32 = 40.0;
pub const GOLD_COLOR: Color = Color::new(1.0, 0.87, 0.0, 1.0);
pub const OUTLINE_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.5);
pub const DRAW_SCORE_X: f32 = 20.0;
pub const DRAW_SCORE_Y: f32 = 40.0;
pub const SONG_ENTRY_HEIGHT: f32 = 40.0;
pub const FONT_SIZE: f32 = 30.0;
pub const COUNTDOWN_DURATION: f64 = 5.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "YumOsu!".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
