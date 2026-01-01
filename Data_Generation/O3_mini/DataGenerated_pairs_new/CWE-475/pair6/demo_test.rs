///////////////////////////////////////////////////////////////////////////////
// Test Oracle:
// This unit test verifies that when an out-of-bound size (20) is provided,
// the duplicate() function must return an error. In the vulnerable implementation,
// no such check exists and the function incorrectly returns Ok(()), causing the 
// test to fail. For the corrected code, the test should pass.
// Note: This test oracle must be compiled and run separately from the main() functions.
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{DataHolder, MemoryOps};

    #[test]
    fn test_invalid_copy_size() {
        let holder = DataHolder::new(10);
        // For a proper implementation, requesting more bytes than allocated should yield an error.
        assert!(holder.duplicate(20).is_err(), "Copy operation should fail for an invalid size");
    }
}