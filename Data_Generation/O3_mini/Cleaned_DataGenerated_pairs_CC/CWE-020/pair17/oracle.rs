#[cfg(test)]
mod tests {
    use super::Processor;
    use std::panic;
    #[test]
    fn oracle() {
        let test_input = "hello".to_string();
        let invalid_length = 10;
        let proc_inst = Processor::new(test_input.clone());
        let vulnerable_outcome = panic::catch_unwind(|| {
            let _ = proc_inst.process(invalid_length);
        });
        if vulnerable_outcome.is_err() {
            panic!("Test failed: Vulnerable version panicked on invalid input.");
        } else {
            let proc_inst_fixed = Processor::new(test_input);
            match proc_inst_fixed.process(invalid_length) {
                Ok(_) => panic!("Test failed: Expected an error for invalid input in fixed version."),
                Err(msg) => assert_eq!(msg, "Input length exceeds data length", "Unexpected error message"),
            }
        }
    }
}
