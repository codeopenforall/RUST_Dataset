/////////////////////////////////////////////////////////////
// Test Oracle Function for CWE-252 Unchecked Return Value
// This test function is intended to be used with both versions.
// It asserts that processing invalid input (i.e., input larger than the allocated
// buffer) returns an error. In the vulnerable version, the error is discarded so the
// test will fail, but in the corrected version, the error is correctly returned and
// the test will pass.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn oracle() {
        // Set up a processor with a fixed-size buffer.
        let data = Arc::new(Mutex::new(vec![0u8; 10]));
        let processor = DataProcessor { data: data.clone() };

        // Valid input: should succeed.
        let res_valid = processor.process(&[1, 2, 3]);
        assert!(res_valid.is_ok(), "Valid input should succeed.");

        // Invalid input: exceeds the buffer size.
        let res_invalid = processor.process(&[1,2,3,4,5,6,7,8,9,10,11]);
        // The oracle expects an error for invalid input.
        assert!(res_invalid.is_err(), "Invalid input should produce an error.");
    }
}