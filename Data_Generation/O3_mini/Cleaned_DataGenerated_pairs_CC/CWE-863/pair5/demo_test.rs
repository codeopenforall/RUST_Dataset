#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Helper function to trigger the worker spawning and then retrieve the shared value.
    fn run_and_get_value(worker_count: usize) -> u8 {
        // Invoke the thread spawning function.
        spawn_workers(worker_count);
        // Access the global resource. In the vulnerable version, the vector is empty and this will panic.
        unsafe {
            if let Some(res) = &GLOBAL_RESOURCE {
                let guard = res.lock().unwrap();
                // Attempt to access the first element; it should have been updated to 42.
                return guard[0];
            } else {
                panic!("Global resource not initialized");
            }
        }
    }

    #[test]
    fn test_resource_value() {
        // Use a small number of workers to avoid overwhelming the system.
        let value = run_and_get_value(5);
        // The expected value is 42.
        assert_eq!(value, 42, "The shared resource value should be updated to 42");
    }
}