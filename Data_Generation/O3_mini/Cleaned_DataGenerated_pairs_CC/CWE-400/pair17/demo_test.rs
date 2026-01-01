#[cfg(test)]
mod tests {
    use super::*;
    
    // A helper function to initialize the global container before tests.
    fn test_initialize() {
        unsafe {
            GLOBAL_DATA = Some(std::sync::Mutex::new(Vec::new()));
        }
    }
    
    // Test oracle: When running with an over-limit number of tasks, the function should return an error.
    #[test]
    fn test_resource_limit() {
        test_initialize();
        let proc_inst = Processor;
        // For the fixed version, passing a count higher than MAX_TASKS should yield an error.
        let res = proc_inst.run(150);
        assert!(res.is_err(), "Expected error when task count exceeds the limit");
        
        // For a safe count, it should succeed.
        test_initialize();
        let res_ok = proc_inst.run(50);
        assert!(res_ok.is_ok(), "Expected success when task count is within the limit");
    }
}