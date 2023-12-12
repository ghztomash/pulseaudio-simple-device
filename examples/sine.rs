extern crate pulseaudio_simple_device as pulse;
use pulse::config::Config;
use pulse::device::Device;
use anyhow::Result;

fn main() -> Result<()> {
    let config = Config::default();
    let device = Device::new("test".to_string());

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(&config, None, err_fn).unwrap();

    stream.play();
    stream.pause();
    drop(stream);

    Ok(())
}
