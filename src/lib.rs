pub mod config;
pub mod device;
pub mod stream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_tests() {
        let config = config::Config::default();
        let device = device::Device::new("test".to_string());
        let _stream = device.build_output_stream(&config, None, None).unwrap();

        assert!(true);
    }
}
