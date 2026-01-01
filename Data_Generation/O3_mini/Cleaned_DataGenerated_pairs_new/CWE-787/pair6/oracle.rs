#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_out_of_bounds() {
        let mut buffer = Buffer::new(10);
        let result = buffer.assign(10, 42);
        assert!(result.is_err(), "Buffer assignment with out-of-bounds index should fail.");
    }
}
