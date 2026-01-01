#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_transformation() {
        // Triggering input that verifies complete and correct copying.
        let data = vec![10, 20, 30, 40];
        let result = Processor::transform(&data);
        // The test asserts that the output matches the input exactly.
        assert_eq!(result, data, "Transformation failed: the output does not match the input");
    }
}