#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Import the MemoryBuffer type from the surrounding code.
    // Both versions expose the same API.
    use super::MemoryBuffer;

    // The test oracle function. It uses deliberately oversized input.
    // In the vulnerable version, the guard will be overwritten and the test will panic.
    // In the corrected version, the function returns an error and preserves the guard.
    #[test]
    fn test_guard_integrity() {
        let input = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let initial_guard: u8 = 0xAA;
        let mem = MemoryBuffer::new(8, initial_guard);
        let shared_mem = Arc::new(Mutex::new(mem));
        let shared_mem_clone = Arc::clone(&shared_mem);

        let handle = std::thread::spawn(move || {
            let mut buffer = shared_mem_clone.lock().unwrap();
            // The behavior differs based on the implementation.
            // For the vulnerable version, write_input does not check the length.
            // For the fixed version, write_input returns Err when input is too long.
            let _ = buffer.write_input(&input);
        });
        handle.join().unwrap();

        let buffer = shared_mem.lock().unwrap();
        assert!(buffer.check_guard(initial_guard), "Guard value corrupted");
    }
}