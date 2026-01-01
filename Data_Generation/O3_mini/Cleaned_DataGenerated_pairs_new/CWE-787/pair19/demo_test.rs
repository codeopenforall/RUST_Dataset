#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::BufferManager;

    #[test]
    fn test_update_sum() {
        // Initialize the BufferManager; initial sum should be 0.
        let initial_manager = BufferManager::new();
        let initial_sum = initial_manager.sum();

        let shared = Arc::new(Mutex::new(initial_manager));
        // Spawn a thread to update the last valid index.
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mgr = shared_clone.lock().unwrap();
            // Update the last element (index 9) with value 42.
            mgr.apply(9, 42).expect("Update operation failed");
        });
        handle.join().unwrap();

        let mgr = shared.lock().unwrap();
        let new_sum = mgr.sum();
        // The contract: The correct behavior updates the sum by exactly 42.
        assert_eq!(new_sum, initial_sum + 42, "Sum did not increase as expected");
    }
}