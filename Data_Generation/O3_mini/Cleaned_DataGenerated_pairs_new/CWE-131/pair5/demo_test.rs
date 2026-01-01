/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let trigger = "overflow";
        let worker = ConcurrentWorker;
        // In the vulnerable code, the miscalculation causes the function to return false,
        // whereas in the fixed code it correctly returns true.
        assert!(
            worker.execute(trigger),
            "Test oracle failed: expected success for properly allocated buffer."
        );
    }
}