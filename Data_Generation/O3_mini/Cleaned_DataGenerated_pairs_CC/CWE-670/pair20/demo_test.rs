//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_behavior() {
        // Initialize TaskManager with a known state.
        let mut tm = TaskManager::new(3);
        
        // When the flag is false, no update should occur.
        tm.execute(0, 100, false).expect("Execution should succeed");
        // Immediately update with flag true: should update index 0 only.
        tm.execute(0, 200, true).expect("Execution should succeed");
        
        // For the corrected code, the expected state is [200, 0, 0].
        // For the vulnerable version, the state would be [100, 200, 0] causing the test to fail.
        assert_eq!(tm.get_tasks(), &[200, 0, 0], "The TaskManager state did not match the expected invariant");
    }
}