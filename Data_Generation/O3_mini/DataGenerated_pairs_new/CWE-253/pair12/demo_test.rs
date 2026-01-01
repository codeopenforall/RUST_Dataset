//////////////////////////////////////////////
// Test Oracle for Verifying Correctness
//////////////////////////////////////////////
#[cfg(test)]
mod test_oracle {
    use super::Processor;

    // This helper function instantiates the Processor and processes the given input,
    // returning the Result as-is.
    fn process_input(input: i32) -> Result<i32, &'static str> {
        let proc_inst = Processor {};
        proc_inst.process(input)
    }

    #[test]
    fn oracle_test() {
        // For a negative input, the correct (fixed) implementation should return an Err.
        // The vulnerable implementation, however, erroneously returns Ok(0).
        let negative_input = -5;
        let res = process_input(negative_input);
        match res {
            Ok(val) => {
                // If the function returns Ok, then the vulnerability is present.
                panic!("Test failed: Negative input produced Ok({}) instead of an error.", val);
            },
            Err(msg) => {
                // The fixed version should reach here.
                assert_eq!(msg, "Negative input not allowed", "Unexpected error message");
            },
        }
    }
}