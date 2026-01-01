use std::sync::{Arc, Mutex};
    state: Arc<Mutex<usize>>,
            state: Arc::new(Mutex::new(64)),
        let handle = thread::spawn(move || {
            let mut bound = state_handle.lock().unwrap();
            if *bound > index {
                *bound = *bound;
            }
        let current_bound = *self.state.lock().unwrap();
            let ptr = self.buffer.as_ptr().add(index);
            assert!(index < current_bound, "Invariant violation: index out of bound");
        handle.join().expect("Thread join failed");
