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

        let err_fn = |_| {};
        let data_fn = |_: &mut [f32]| {};

        let _stream = device
            .build_output_stream(&config, data_fn, err_fn, None)
            .unwrap();

        assert!(true);
    }
}
