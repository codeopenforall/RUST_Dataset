#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
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
