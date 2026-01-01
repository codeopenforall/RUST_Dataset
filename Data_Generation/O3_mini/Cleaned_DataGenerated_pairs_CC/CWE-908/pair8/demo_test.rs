//////////////// Oracle Test Code Begin
// This is the test oracle function. It must be compiled separately and linked against either the vulnerable or fixed code.
// It asserts that the configuration is properly initialized (i.e. cfg.valid is true) and that the thread computations yield the expected result (42 + 1 = 43).
// Note: This file must not be merged with the main binary builds.

#[cfg(test)]
mod tests {
    use super::*;

    // A helper function to mimic the production processing.
    fn aggregate_result() -> Vec<u32> {
        run_threads()
    }

    #[test]
    fn test_config_integrity() {
        // Directly test the configuration load.
        let cfg = load_config();
        assert!(cfg.valid, "Configuration 'valid' flag is not properly initialized!");
    }

    #[test]
    fn test_thread_results() {
        let results = aggregate_result();
        // The expected result for each thread is: if valid is true, then value + 1, i.e. 43.
        for res in results {
            assert_eq!(res, 43, "Thread computation produced an unexpected result");
        }
    }
}
//////////////// Oracle Test Code End