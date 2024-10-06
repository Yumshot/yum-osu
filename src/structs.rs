// src/structs.rs

use macroquad::prelude::{ Texture2D, Vec2 };
use std::time::Instant;
use std::sync::mpsc;
use rodio::Decoder;
use std::io::BufReader;
use std::fs::File;

pub struct Assets {
    pub menu_background: Texture2D,
    pub start_button: Texture2D,
}

pub struct SongSelectionState {
    pub scroll_pos: f32,
    pub selected_song: Option<String>,
}

pub enum GameState {
    Menu,
    SongSelection,
    Playing,
    Loading {
        rx: mpsc::Receiver<Vec<f64>>,
        start_time: Instant,
    },
    ReadyToPlay {
        beats: Vec<f64>,
        ready_time: Instant,
        source: Option<Decoder<BufReader<File>>>,
    },
    Visualizing(Box<VisualizingState>),
    End,
}

pub struct Circle {
    pub position: Vec2,
    pub spawn_time: f64,
    pub hit_time: f64,
    pub max_radius: f32,
    pub hit: bool,
    pub missed: bool,
}

pub struct FloatingText {
    pub text: String,
    pub position: Vec2,
    pub spawn_time: f64,
    pub duration: f64,
}

pub struct VisualizingState {
    pub beats: Vec<f64>,
    pub start_time: Instant,
    pub circles: Vec<Circle>,
    pub score: i32,
    pub floating_texts: Vec<FloatingText>,
}
