#[cfg(test)]
mod tests {
    use super::*;
    fn aggregate_result() -> Vec<u32> {
        run_threads()
    }
    #[test]
    fn test_config_integrity() {
        let cfg = load_config();
        assert!(cfg.valid, "Configuration 'valid' flag is not properly initialized!");
    }
    #[test]
    fn test_thread_results() {
        let results = aggregate_result();
        for res in results {
            assert_eq!(res, 43, "Thread computation produced an unexpected result");
        }
    }
}
