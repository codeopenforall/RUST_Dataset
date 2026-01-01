use std::sync::Arc;
        let shared = Arc::new(42);
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            *shared_clone
        });
        handle.join().unwrap()
