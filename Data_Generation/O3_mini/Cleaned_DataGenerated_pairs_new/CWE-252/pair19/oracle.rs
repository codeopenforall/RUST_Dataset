#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn invariant_test() {
        let holder = Arc::new(Mutex::new(Holder { data: 0 }));
        let holder_clone = Arc::clone(&holder);
        let handle = thread::spawn(move || {
            let mut h = holder_clone.lock().unwrap();
            let _ = h.modify(-10);
        });
        handle.join().unwrap();
        let final_value = holder.lock().unwrap().data;
        assert!(final_value >= 0, "Invariant violated: data is negative");
    }
}
