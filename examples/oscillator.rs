extern crate pulseaudio_simple_device as pulse;
use anyhow::Result;
use pulse::config::Config;
use pulse::device::Device;
use std::io::stdin;
use std::sync::{Arc, Mutex};

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

struct Oscillator {
    sample_rate: f32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    phase_increment: f32,
}

impl Oscillator {
    fn new(sample_rate: f32, frequency: f32) -> Self {
        let mut osc = Oscillator {
            sample_rate,
            frequency: 0.0,
            amplitude: 1.0,
            phase: 0.0,
            phase_increment: 0.0,
        };
        osc.set_frequency(frequency);
        return osc;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase_increment = (self.frequency * TWO_PI) / self.sample_rate;
    }

    fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    fn process(&mut self) -> f32 {
        let sample = self.amplitude * self.phase.sin();
        self.phase = (self.phase + self.phase_increment) % TWO_PI;
        return sample;
    }
}

fn main() -> Result<()> {
    let config = Config::default();
    let device = Device::new("test".to_string());

    let channels = config.channels as usize;
    let sample_rate = config.sample_rate as f32;

    // Create a sine wave generator.
    let osc = Arc::new(Mutex::new(Oscillator::new(sample_rate, 440.0)));
    let thread_osc = Arc::clone(&osc);

    // callbacks
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let data_fn = move |data: &mut [f32]| {
        for frame in data.chunks_mut(channels) {
            // copy the same value to all channels
            let value = thread_osc.lock().unwrap().process();
            for sample in frame {
                *sample = value;
            }
        }
    };

    let stream = device.build_output_stream(&config, data_fn, err_fn)?;
    stream.play()?;

    // wait for user input
    println!("enter q to quit");
    loop {
        let input = stdin().lines().next().unwrap().unwrap();
        match input.trim() {
            "f" => {
                osc.lock().unwrap().set_frequency(880.0);
            }
            "a" => {
                osc.lock().unwrap().set_amplitude(0.5);
            }
            "q" => {
                println!("got q, exiting...");
                break;
            }
            _ => {}
        }
    }

    stream.pause()?;
    drop(stream);
    Ok(())
}
