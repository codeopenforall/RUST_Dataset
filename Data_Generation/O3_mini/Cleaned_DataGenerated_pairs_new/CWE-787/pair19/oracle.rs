#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::BufferManager;
    #[test]
    fn test_update_sum() {
        let initial_manager = BufferManager::new();
        let initial_sum = initial_manager.sum();
        let shared = Arc::new(Mutex::new(initial_manager));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mgr = shared_clone.lock().unwrap();
            mgr.apply(9, 42).expect("Update operation failed");
        });
        handle.join().unwrap();
        let mgr = shared.lock().unwrap();
        let new_sum = mgr.sum();
        assert_eq!(new_sum, initial_sum + 42, "Sum did not increase as expected");
    }
}
