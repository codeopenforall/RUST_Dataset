#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn test_input_validation() {
        let proc_inst = Processor::new();
        let result = proc_inst.process("15");
        assert_eq!(result, "Invalid input size", "The process function did not validate the input length properly");
    }
}
