//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::DataBuffer;

    #[test]
    fn test_operation() {
        // For the vulnerable variant, run_task() returns false because of the unchecked error.
        // For the corrected variant, run_task() returns true after proper handling.
        assert!(
            DataBuffer::run_task(),
            "The operation did not complete successfully as expected"
        );
    }
}