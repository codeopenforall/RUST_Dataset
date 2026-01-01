#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    fn run_app() -> u8 {
        let block = Arc::new(Mutex::new(super::MemoryBlock { data: [0; 10], flag: 0 }));
        let block_clone = Arc::clone(&block);
        let handle = thread::spawn(move || {
            let mut guarded = block_clone.lock().unwrap();
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
        assert_eq!(result, 0, "Error: The adjacent field was unexpectedly modified.");
    }
}
