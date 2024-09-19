use macroquad::prelude::*;
use rodio::{ Decoder, OutputStream, Sink, Source };
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;
use ::rand::prelude::*;

struct Circle {
    position: Vec2,
    spawn_time: f64,
    hit_time: f64,
    max_radius: f32,
}

#[macroquad::main("Rhythm Visualizer")]
async fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let (beats, duration, bpm) = gather_beats("src/assets/music/hardy.mp3", &sink);

    // Use a time window-based filtering with max hits
    let filtered_beats = filter_beats_with_limit(&beats, bpm, 3, 1.0); // Max 3 hits per 1 second

    let start_time = Instant::now();
    visualize_pattern(&filtered_beats, start_time, &sink).await;
}

async fn visualize_pattern(beats: &[f64], start_time: Instant, sink: &Sink) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let max_circle_radius = 100.0;
    let shrink_time = 1.5;

    let mut rng = ::rand::thread_rng();
    let mut circles: Vec<Circle> = beats
        .iter()
        .map(|&beat_time| {
            let position = Vec2::new(
                rng.gen_range(50.0..screen_width - 50.0),
                rng.gen_range(50.0..screen_height - 50.0)
            );

            Circle {
                position,
                spawn_time: beat_time - shrink_time,
                hit_time: beat_time,
                max_radius: max_circle_radius,
            }
        })
        .collect();

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();

        clear_background(WHITE);

        for circle in &circles {
            let time_since_spawn = elapsed - circle.spawn_time;

            if time_since_spawn >= 0.0 && time_since_spawn <= shrink_time {
                let scale_factor = 1.0 - time_since_spawn / shrink_time;
                let radius = circle.max_radius * (scale_factor as f32);
                draw_circle(circle.position.x, circle.position.y, radius, BLUE);
            }
        }

        if sink.empty() {
            break;
        }

        next_frame().await;
    }
}

fn gather_beats(path: &str, sink: &Sink) -> (Vec<f64>, f32, f64) {
    let file_for_analysis = File::open(path).expect("Failed to open MP3 file");
    let reader_for_analysis = BufReader::new(file_for_analysis);
    let source_for_analysis = Decoder::new(reader_for_analysis).expect("Failed to decode MP3");

    let samples: Vec<f32> = source_for_analysis.convert_samples::<f32>().collect();
    let beats = detect_beats(&samples, 44100);
    let bpm = calculate_bpm(&beats);

    let file_for_playback = File::open(path).expect("Failed to open MP3 file");
    let reader_for_playback = BufReader::new(file_for_playback);
    let source_for_playback = Decoder::new(reader_for_playback).expect("Failed to decode MP3");

    let duration = source_for_playback.total_duration().unwrap_or_default().as_secs_f32();
    sink.append(source_for_playback);
    sink.play();

    (beats, duration, bpm)
}
fn calculate_bpm(beats: &[f64]) -> f64 {
    if beats.len() < 2 {
        return 0.0;
    }

    let mut intervals = Vec::new();
    for window in beats.windows(2) {
        let interval = window[1] - window[0];
        intervals.push(interval);
    }

    let average_interval = intervals.iter().sum::<f64>() / (intervals.len() as f64);
    let bpm = 60.0 / average_interval;
    bpm
}

fn filter_beats(beats: &[f64], bpm: f64) -> Vec<f64> {
    let half_note_interval = 60.0 / (bpm * 2.0); // Time between half notes (reduces number of hits)
    let mut filtered_beats = Vec::new();
    let mut last_beat = 0.0;

    for &beat in beats {
        if beat - last_beat >= half_note_interval {
            filtered_beats.push(beat);
            last_beat = beat;
        }
    }

    filtered_beats
}

fn filter_beats_with_limit(
    beats: &[f64],
    bpm: f64,
    max_hits_per_window: usize,
    time_window: f64
) -> Vec<f64> {
    let mut filtered_beats = Vec::new();
    let mut hits_in_window = 0;
    let mut last_window_start = 0.0;

    for &beat in beats {
        // If we're outside the current window, reset the hit counter
        if beat - last_window_start >= time_window {
            last_window_start = beat;
            hits_in_window = 0;
        }

        // Only add beats if we haven't hit the maximum for this window
        if hits_in_window < max_hits_per_window {
            filtered_beats.push(beat);
            hits_in_window += 1;
        }
    }

    filtered_beats
}

fn detect_beats(samples: &[f32], sample_rate: u32) -> Vec<f64> {
    let mut beats = Vec::new();
    let mut prev_sample: f32 = 0.0;

    for (i, &sample) in samples.iter().enumerate() {
        if sample.abs() > 0.8 && prev_sample.abs() < 0.8 {
            let time_in_seconds = (i as f64) / (sample_rate as f64);
            beats.push(time_in_seconds);
        }
        prev_sample = sample;
    }

    beats
}
