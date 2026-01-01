//////////////////////
// Test Oracle Code
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test verifies that when provided with out-of-bound parameters for the input string,
    // the safe implementation correctly returns an error.
    // For the vulnerable version, this test is expected to fail (panic or undefined behavior).
    #[test]
    fn test_oracle() {
        // "12345" has length 5; parameters (start=3, length=5) result in end=8, which is out-of-bound.
        let input_data = "12345".to_string();
        let manager = BufferManager { data: input_data.into_bytes() };
        let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
        let result = proc_inst.run(3, 5);
        assert!(result.is_err(), "Expected an error due to out-of-bound extraction");
        assert_eq!(result.err().unwrap(), "out of bounds");
    }
}