use std::sync::{Arc, Mutex};
const MAX_CAPACITY: usize = 1000;
    data: Mutex<Vec<i32>>,
            data: Mutex::new(Vec::new()),
        let mut vec = self.data.lock().unwrap();
        if vec.len() < MAX_CAPACITY {
            vec.push(value);
        let vec = self.data.lock().unwrap();
        vec.len()
    count <= MAX_CAPACITY
