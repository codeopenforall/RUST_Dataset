#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::DataBlock;
    #[test]
    fn test_buffer() {
        const SIZE: usize = 10;
        let block = Arc::new(Mutex::new(DataBlock::new(SIZE)));
        let block_clone = Arc::clone(&block);
        let handle = thread::spawn(move || {
            let mut db = block_clone.lock().unwrap();
            db.fill();
        });
        handle.join().unwrap();
        let db = block.lock().unwrap();
        for i in 0..db.data.len() {
            assert_eq!(db.data[i], i as u32, "Buffer verification failed at index {}", i);
        }
    }
}
