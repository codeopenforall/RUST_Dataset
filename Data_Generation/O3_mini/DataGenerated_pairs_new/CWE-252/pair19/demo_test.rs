#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // This test verifies the invariant that the internal data must remain non-negative.
    // In the vulnerable version the unchecked return causes the data to be updated to a negative value,
    // leading the assertion to fail. In the corrected version, the update is rejected, and the invariant holds.
    #[test]
    fn invariant_test() {
        let holder = Arc::new(Mutex::new(Holder { data: 0 }));
        let holder_clone = Arc::clone(&holder);
        let handle = thread::spawn(move || {
            let mut h = holder_clone.lock().unwrap();
            // Trigger an update with a negative value.
            let _ = h.modify(-10);
        });
        handle.join().unwrap();
        let final_value = holder.lock().unwrap().data;
        assert!(final_value >= 0, "Invariant violated: data is negative");
    }
}