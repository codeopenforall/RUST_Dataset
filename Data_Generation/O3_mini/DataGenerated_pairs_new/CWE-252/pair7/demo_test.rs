#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    // Test oracle function: In the fixed version, out-of-bound updates are rejected
    // and do not corrupt the vector. In the vulnerable version, the unchecked operations
    // may corrupt adjacent memory. This test should pass only for the secured implementation.
    #[test]
    fn integrity_check() {
        let engine = Core::new(10);
        
        // Valid update: set index 5 to 77.
        assert!(engine.update(5, 77).is_ok(), "Valid update failed");
        
        // Trigger an out-of-bound update concurrently.
        let _ = engine.concurrent_update(15, 88);
        thread::sleep(Duration::from_millis(100));
        
        // Reacquire the data safely.
        let data = engine.data.lock().unwrap();
        
        // Confirm that the valid update remains intact.
        assert_eq!(data[5], 77, "Valid update was unexpectedly altered");
        
        // Ensure that the out-of-bound update did not inadvertently modify any in-bounds element.
        if data.iter().any(|&x| x == 88) {
            panic!("Memory corruption detected: out-of-bound update modified valid memory");
        }
    }
}