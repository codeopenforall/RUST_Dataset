#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::MemoryBuffer;
    #[test]
    fn test_guard_integrity() {
        let input = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let initial_guard: u8 = 0xAA;
        let mem = MemoryBuffer::new(8, initial_guard);
        let shared_mem = Arc::new(Mutex::new(mem));
        let shared_mem_clone = Arc::clone(&shared_mem);
        let handle = std::thread::spawn(move || {
            let mut buffer = shared_mem_clone.lock().unwrap();
            let _ = buffer.write_input(&input);
        });
        handle.join().unwrap();
        let buffer = shared_mem.lock().unwrap();
        assert!(buffer.check_guard(initial_guard), "Guard value corrupted");
    }
}
