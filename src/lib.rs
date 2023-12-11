mod config;
mod device;
mod stream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_tests() {
        let config = config::Config::default();
        let device = device::Device::new(config);
        let stream = device.build_output_stream();

        assert_eq!(1, 1);
    }
}
