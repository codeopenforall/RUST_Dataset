#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn test_process() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![2, 3, 4, 5];
        let result = Processor::process(&input);
        assert_eq!(result, expected, "Each element must be incremented by 1 and the vector should have no extra element");
    }
}
