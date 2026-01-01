#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn test_processor_sum() {
        let proc = Processor;
        let input = vec![1, 2, 3, 4];
        let expected: u32 = 10; 
        let res = proc.execute(&input);
        assert_eq!(res, expected, "The computed sum does not match the expected value.");
    }
}
