use std::sync::{Arc, Mutex};
    counter: Mutex<u32>,
            counter: Mutex::new(0),
                let mut guard = data.counter.lock().unwrap();
                if *guard < 10000 {
                    thread::sleep(Duration::from_micros(1));
                    *guard += 1;
    let final_val = *shared.counter.lock().unwrap();
    final_val
