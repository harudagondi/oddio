use std::{thread, time::Duration};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const DURATION_SECS: u32 = 6;
const BUFFER_SIZE_MS: u32 = 100;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let sample_rate = device.default_output_config().unwrap().sample_rate();
    let config = cpal::StreamConfig {
        channels: 2,
        sample_rate,
        buffer_size: cpal::BufferSize::Fixed(sample_rate.0 / (1000 / BUFFER_SIZE_MS)),
    };
    let boop = oddio::Sound::from_iter(
        sample_rate.0,
        // Generate a simple sine wave
        (0..sample_rate.0 * DURATION_SECS).map(|i| {
            let t = i as f32 / sample_rate.0 as f32;
            (t * 500.0 * 2.0 * std::f32::consts::PI).sin() * 8.0
        }),
    );

    let speed = 50.0;
    let mut boop_state = oddio::State::new([-speed, 10.0, 0.0].into());
    let mut sample = 0;
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let samples = oddio::aggregate_stereo(data);
                sample += samples.len();
                let t = sample as f64 / sample_rate.0 as f64;
                let mut mixer = oddio::Mixer::new(sample_rate.0, samples);
                mixer.mix(oddio::Input {
                    sound: &boop,
                    t,
                    state: &mut boop_state,
                    position_wrt_listener: [-speed + speed * t as f32, 10.0, 0.0].into(),
                });
            },
            move |err| {
                eprintln!("{}", err);
            },
        )
        .unwrap();
    stream.play().unwrap();
    thread::sleep(Duration::from_secs(DURATION_SECS as u64));
}
