#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn test_transformation() {
        let data = vec![10, 20, 30, 40];
        let result = Processor::transform(&data);
        assert_eq!(result, data, "Transformation failed: the output does not match the input");
    }
}
