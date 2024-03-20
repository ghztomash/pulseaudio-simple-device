extern crate pulseaudio_simple_device as pulse;
use anyhow::Result;
use pulse::config::Config;
use pulse::device::Device;
use std::thread;
use std::time::Duration;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

fn main() -> Result<()> {
    let config = Config::default();
    let device = Device::new("test".to_string());

    let channels = config.channels as usize;
    let phase_inc = (440.0 * TWO_PI) / config.sample_rate as f32;
    let mut phase: f32 = 0.0;

    // callbacks
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let data_fn = move |data: &mut [f32]| {
        for frame in data.chunks_mut(channels) {
            for sample in frame {
                *sample = (phase).sin();
            }
            phase += phase_inc;
        }
    };

    let stream = device.build_output_stream(&config, data_fn, err_fn, None)?;
    stream.play()?;

    thread::sleep(Duration::from_secs(2));
    stream.pause()?;
    thread::sleep(Duration::from_secs(1));

    drop(stream);
    Ok(())
}
