//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // Test to verify that the counter reaches the expected final value.
    // For the vulnerable version this test is expected to fail intermittently 
    // due to the race condition, while the fixed version reliably passes.
    #[test]
    fn check_counter() {
        let shared = Arc::new(Data::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let local = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        let final_value = shared.fetch();
        // The expected value after 10 threads each incrementing 1000 times.
        assert_eq!(final_value, 10000, "Detected race condition: final value is {}, expected 10000", final_value);
    }
}