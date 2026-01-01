use std::sync::{Arc, Mutex};
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut data = shared_clone.lock().unwrap();
            if data.count % 2 == 0 {
                thread::sleep(Duration::from_millis(10));
                data.count += 1;
            } else {
                thread::sleep(Duration::from_millis(10));
                data.count += 2;
    let final_data = shared.lock().unwrap();
    final_data.count
    println!("Final count (synchronized): {}", result);
