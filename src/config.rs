pub struct Config {
    pub channels: u8,
    pub sample_rate: u32,
    pub buffer_size: usize,
}

impl Config {
    pub fn new(channels: u8, sample_rate: u32, buffer_size: usize) -> Self {
        Config {
            channels,
            sample_rate,
            buffer_size,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            channels: 2,
            sample_rate: 44100,
            buffer_size: 1024,
        }
    }
}
