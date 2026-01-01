use std::{sync::{Arc, Mutex}, thread};
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
        let shared_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
                let mut data = shared_clone.lock().unwrap();
                let current = data.count;
                thread::yield_now();
                data.count = current + 1;
    let result = shared.lock().unwrap().count;
