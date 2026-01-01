use std::sync::{Arc, Mutex};
    let shared = Arc::new(Mutex::new(0));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
                let mut counter = shared_clone.lock().unwrap();
                *counter += 1;
    let counter = shared.lock().unwrap();
    *counter
