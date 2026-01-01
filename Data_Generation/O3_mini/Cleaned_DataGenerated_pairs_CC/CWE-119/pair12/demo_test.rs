//////////////// Test Oracle Function ////////////////////
// The following unit test ensures that the adjacent field remains unmodified (i.e., remains 0).
// In the vulnerable version, the unsafe update would corrupt the field (setting it to 255), causing the test to fail.
// In the corrected version, the boundary check prevents any modifications, so the test passes.

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // We replicate the application logic in a helper function that returns the value of the flag.
    // Note: This helper is used solely for testing and is not part of the main application.
    fn run_app() -> u8 {
        // Create the memory block with a fixed array and adjacent field.
        let block = Arc::new(Mutex::new(super::MemoryBlock { data: [0; 10], flag: 0 }));
        let block_clone = Arc::clone(&block);
        let handle = thread::spawn(move || {
            let mut guarded = block_clone.lock().unwrap();
            // The update call with an out-of-bound index is intended to either corrupt the flag (vulnerable)
            // or be safely ignored (corrected).
            // For the vulnerable version, this unsafe call mutates memory incorrectly.
            // For the approved version, internal index check prevents any corruption.
            #[allow(unused_unsafe)]
            unsafe { guarded.update(10, 255); }
        });
        handle.join().unwrap();
        let guarded = block.lock().unwrap();
        guarded.flag
    }

    #[test]
    fn oracle_test() {
        let result = run_app();
        // The expected behavior is that the flag field remains unmodified (0).
        // In the vulnerable variant, the flag is altered to 255.
        assert_eq!(result, 0, "Error: The adjacent field was unexpectedly modified.");
    }
}