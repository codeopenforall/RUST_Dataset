use std::sync::{Arc, Mutex};
    count: Mutex<i32>,
            count: Mutex::new(val),
        let mut num = self.count.lock().unwrap();
        *num += 1;
        *self.count.lock().unwrap()
