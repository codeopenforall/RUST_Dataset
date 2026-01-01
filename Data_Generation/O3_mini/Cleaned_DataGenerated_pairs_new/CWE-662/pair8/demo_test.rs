#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test spawns two threads that update the shared data.
    // Each update increments two counters. Thus, after 2000 updates,
    // the expected aggregate sum is 4000.
    #[test]
    fn test_computed_sum() {
        let shared = Arc::new(Shared::new());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let clone = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    clone.update();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let result = shared.finalize();
        // Passing condition: result should equal 4000.
        // In the vulnerable version, result becomes 3999 (off-by-one error), triggering a test failure.
        assert_eq!(result, 4000, "Computed aggregate sum is incorrect");
    }
}