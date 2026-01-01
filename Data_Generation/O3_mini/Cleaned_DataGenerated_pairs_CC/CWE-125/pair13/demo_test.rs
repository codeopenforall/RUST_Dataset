#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        // Create processor instance with a fixed buffer.
        let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
        // Using a range that exceeds the buffer bounds: start 2 with count 4 (2 + 4 > 5).
        let result = std::panic::catch_unwind(|| {
            proc.compute(2, 4)
        });
        // The fixed implementation panics on out-of-bounds access.
        assert!(result.is_err(), "Expected panic due to out-of-bound access");
    }
}