extern crate pulseaudio_simple_device as pulse;
use pulse::config::Config;
use pulse::device::Device;
use anyhow::Result;

fn main() -> Result<()> {
    let config = Config::default();
    let device = Device::new("test".to_string());
    let stream = device.build_output_stream(&config, None, None).unwrap();

    stream.play();
    stream.pause();
    drop(stream);

    Ok(())
}
