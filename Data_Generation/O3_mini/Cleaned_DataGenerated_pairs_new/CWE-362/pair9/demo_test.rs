#[cfg(test)]
mod tests {
    use super::run_state;

    #[test]
    fn test_state() {
        // Invoke the simulation that spawns concurrent threads.
        let (counter, finished) = run_state();
        // The contract is that after processing, exactly 10 increments should have occurred 
        // and the finished flag must be true.
        assert_eq!(counter, 10, "Expected counter to be exactly 10");
        assert!(finished, "Expected finished flag to be true");
    }
}