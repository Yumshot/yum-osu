use aubio::{Onset, OnsetMode};
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub async fn gather_beats(path: &str, sink: &Sink) -> Vec<f64> {
    let file = File::open(path).expect("Failed to open audio file");
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader).expect("Failed to decode audio");
    let sample_rate = decoder.sample_rate();

    let samples: Vec<f32> = decoder.convert_samples().collect();

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

        if onset.do_result(&buffer).unwrap() > 0.0 {
            let onset_time = onset.get_last_s();
            beats.push(onset_time as f64);
        }

        position += hop_size;
    }

    beats
}
