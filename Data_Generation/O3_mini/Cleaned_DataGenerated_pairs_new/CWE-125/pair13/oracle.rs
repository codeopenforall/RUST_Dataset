#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
        let result = std::panic::catch_unwind(|| {
            proc.compute(2, 4)
        });
        assert!(result.is_err(), "Expected panic due to out-of-bound access");
    }
}
