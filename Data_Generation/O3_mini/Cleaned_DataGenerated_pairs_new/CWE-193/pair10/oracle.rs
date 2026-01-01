#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_data_processing() {
        let input = b"abcdef"; 
        let output = run(input);
        assert_eq!(output.len(), input.len(), "Output length mismatch");
        assert_eq!(output, input.to_vec(), "Output content mismatch");
    }
}
