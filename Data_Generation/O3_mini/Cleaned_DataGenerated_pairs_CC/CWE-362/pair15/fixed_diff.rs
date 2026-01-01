use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(Counter::new()));
        let counter_clone = Arc::clone(&counter);
                {
                    let mut data = counter_clone.lock().unwrap();
                    data.count = data.count.wrapping_add(1);
                thread::sleep(Duration::from_micros(1));
    let data = counter.lock().unwrap();
    data.count
