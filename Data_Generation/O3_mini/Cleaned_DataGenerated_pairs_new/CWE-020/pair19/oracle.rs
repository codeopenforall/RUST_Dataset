#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extraction_validation() {
        let content = "Hello, World!";
        let proc_inst = Processor::new(content);
        let result = proc_inst.process(7, 10);
        assert!(result.is_err(), "Expected error due to out-of-bound indices");
    }
}
