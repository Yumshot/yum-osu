use rodio::{ Decoder, Source };
use std::fs::File;
use std::io::BufReader;
use aubio::{ Onset, OnsetMode };
use biquad::{ Biquad, Coefficients, DirectForm1, ToHertz, Type as FilterType, Q_BUTTERWORTH_F32 };

/// Read an audio file and find the times of the kick beats
pub fn gather_beats(path: &str) -> Vec<f64> {
    println!("Loading audio file: {}", path);
    // Open the file
    let file = File::open(path).expect("Failed to open audio file");

    // Create a reader that buffers the file
    let reader = BufReader::new(file);

    // Decode the audio from the reader
    let decoder = Decoder::new(reader).expect("Failed to decode audio");

    // Get the sample rate of the audio
    let sample_rate = decoder.sample_rate();

    // Collect all of the samples from the audio
    let samples: Vec<f32> = decoder.convert_samples().collect();

    // Find the kick beats in the samples
    let beats = detect_kick_beats(&samples, sample_rate);
    beats
}

/// Find the kick beats in a set of samples
fn detect_kick_beats(samples: &[f32], sample_rate: u32) -> Vec<f64> {
    // The size of the buffer for the band-pass filter
    let buffer_size = 1024;
    // The size of the hop (how much to move the buffer along the samples)
    let hop_size = 512;

    // The frequency of the kick drum
    let freq = 100.0;
    // The quality factor for the filter
    let q_factor = Q_BUTTERWORTH_F32;

    // Create the band-pass filter
    let bandpass_coefficients = Coefficients::<f32>
        ::from_params(FilterType::BandPass, sample_rate.hz(), freq.hz(), q_factor)
        .unwrap();

    // Initialize the band-pass filter
    let mut bandpass_filter = DirectForm1::<f32>::new(bandpass_coefficients);

    // Apply the band-pass filter to the samples
    let filtered_samples: Vec<f32> = samples
        .iter()
        .map(|&sample| bandpass_filter.run(sample))
        .collect();

    // Initialize the onset detection
    let mut onset = Onset::new(OnsetMode::Energy, buffer_size, hop_size, sample_rate).unwrap();
    // Set the threshold for the onset detection
    onset.set_threshold(0.5);
    // Set the silence for the onset detection
    onset.set_silence(-40.0);

    // Initialize the vector to store the kick beats
    let mut beats = Vec::new();
    // Initialize the buffer to store the samples for the onset detection
    let mut buffer = vec![0.0; buffer_size];
    // Initialize the position in the samples
    let mut position = 0;

    // Loop through the samples and detect onsets
    while position + buffer_size <= filtered_samples.len() {
        // Copy the samples into the buffer
        buffer.copy_from_slice(&filtered_samples[position..position + buffer_size]);

        // Check for an onset
        if onset.do_result(&buffer).unwrap() > 0.0 {
            // Get the time of the onset
            let onset_time = onset.get_last_s();
            // Add the time to the vector of beats
            beats.push(onset_time as f64);
        }
        // Move the position along the samples
        position += hop_size;
    }

    // Return the vector of beats
    beats
}
