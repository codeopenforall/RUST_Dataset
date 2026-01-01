use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
    let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
    counter.load(Ordering::SeqCst)
