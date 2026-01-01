#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // This test oracle is designed to work with the common API defined in both versions.
    // Both implementations expose a method named update() with the same signature.
    // The test invokes update() with an out-of-bounds index and expects an error.
    // In the vulnerable version, the index is not checked and the call returns Ok(()),
    // causing the test to fail. In the fixed version, the out-of-bound condition is detected,
    // an error is returned, and the test passes.

    #[test]
    fn oracle_test() {
        let wrapper = Arc::new(Mutex::new(crate::BufferWrapper::new(10)));
        let wrapper_clone = Arc::clone(&wrapper);
        let handle = thread::spawn(move || {
            let mut lock = wrapper_clone.lock().unwrap();
            lock.update(15, 55)
        });
        let result = handle.join().unwrap();
        assert!(result.is_err(), "Out-of-bounds write should be rejected.");
    }
}