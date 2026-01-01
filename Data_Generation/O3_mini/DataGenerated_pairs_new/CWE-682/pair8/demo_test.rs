#[cfg(test)]
mod tests {
    use crate::calc::Processor;

    #[test]
    fn validate_computation() {
        let processor = Processor::new();
        // For input [100, 80], the correct error percentage should be ((100 - 80) * 100) / 100 = 20.
        let input_data = vec![100u64, 80u64];
        let result = processor.execute(&input_data);
        assert_eq!(result, 20, "Computation error: expected 20 but got {}", result);
    }
}