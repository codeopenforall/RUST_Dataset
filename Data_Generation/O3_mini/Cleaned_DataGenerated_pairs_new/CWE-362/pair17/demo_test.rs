//////////////////////////////////////////////
// Test Oracle for Race Condition Issue
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // We assume that the binary being tested exports a function `acquire_resource`
    // that returns a pointer (as usize) to the singleton resource.
    // The oracle spawns multiple threads that concurrently call the function and then
    // asserts that all returned pointer addresses are identical.

    // Import the function from the target module. Adjust the module name as needed.
    use super::acquire_resource;

    #[test]
    fn test_singleton_invariant() {
        // Create a channel to collect pointer addresses.
        let (tx, rx) = mpsc::channel();

        // Spawn several threads.
        let mut handles = Vec::new();
        for _ in 0..10 {
            let tx_clone = tx.clone();
            handles.push(thread::spawn(move || {
                // Small sleep to simulate realistic thread interleaving.
                thread::sleep(Duration::from_millis(10));
                let ptr = acquire_resource() as usize;
                tx_clone.send(ptr).expect("Failed to send pointer");
            }));
        }
        drop(tx);

        // Collect all pointer values.
        let pointers: Vec<usize> = rx.iter().collect();

        // Wait for all threads to finish.
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Assert that all pointers are equal.
        let first = pointers.first().expect("No pointers collected");
        for ptr in pointers.iter() {
            assert_eq!(ptr, first, "Multiple initializations detected: {} vs {}", ptr, first);
        }
    }
}