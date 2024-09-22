use macroquad::prelude::Vec2;

#[derive(Debug)]
pub struct Circle {
    pub position: Vec2,
    pub spawn_time: f64,
    pub hit_time: f64,
    pub max_radius: f32,
    pub hit: bool,
}
