#[cfg(test)]
mod tests {
    use super::Processor;

    // This oracle test calls the process method with a length string ("15")
    // On the vulnerable version, this would lead to undefined behavior or incorrect results.
    // On the corrected version, it will return the error message "Invalid input size".
    #[test]
    fn test_input_validation() {
        let proc_inst = Processor::new();
        let result = proc_inst.process("15");
        // The test expects the properly validated response.
        assert_eq!(result, "Invalid input size", "The process function did not validate the input length properly");
    }
}