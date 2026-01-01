///////////////////////////////////////////////////////////////////////////////
// Test Oracle: This function is used in a unit test environment. It creates an 
// instance of the service and attempts to submit 1100 tasks. In the fixed version, 
// once the queue limit (MAX_TASKS) is reached, submit() returns an error and the 
// test asserts that error detection occurred. In the vulnerable version, all tasks 
// are accepted, causing the test to fail.
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_limit() {
        let svc = Service::new();
        let mut error_detected = false;
        // Try to submit slightly above the maximum limit.
        for i in 0..1100 {
            if svc.submit(format!("Task {}", i)).is_err() {
                error_detected = true;
                break;
            }
        }
        assert!(error_detected, "Expected rejection when queue limit is exceeded");
    }
}