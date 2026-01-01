#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_validation() {
        // Use an input string known to be 13 bytes long.
        let content = "Hello, World!";
        let proc_inst = Processor::new(content);

        // The chosen indices (offset=7, length=10) exceed the string bounds.
        // The test expects an error.
        let result = proc_inst.process(7, 10);
        assert!(result.is_err(), "Expected error due to out-of-bound indices");
    }
}