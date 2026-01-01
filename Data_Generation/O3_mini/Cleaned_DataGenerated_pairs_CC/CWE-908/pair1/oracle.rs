#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_configuration_integrity() {
        #[allow(unused_unsafe)]
        let config = unsafe { Settings::load() };
        let sum = config.timeout + config.retries as u64;
        assert_eq!(sum, 33, "Configuration fields did not sum to the expected value");
    }
}
