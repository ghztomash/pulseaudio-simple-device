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

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let _stream = device.build_output_stream(&config, None, err_fn).unwrap();

        assert!(true);
    }
}
