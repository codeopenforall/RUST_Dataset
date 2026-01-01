#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_input() {
        let config = Config::new();
        let result = unsafe { config.process("10") };
        assert!(result.is_err(), "Expected an error for out-of-bounds index");
        if let Err(e) = result {
            assert_eq!(e, "Index out of bounds".to_string(), "Error message must indicate bounds violation");
        }
    }
}
