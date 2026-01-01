use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
    state: Arc<AtomicUsize>,
            state: Arc::new(AtomicUsize::new(64)),
        thread::spawn(move || {
            state_handle.store(0, Ordering::Relaxed); 
            let ptr = self.buffer.as_ptr().add(index); 
            let current_bound = self.state.load(Ordering::Relaxed);
            assert!(index < current_bound, "Invariant violation: index out of bound"); 
