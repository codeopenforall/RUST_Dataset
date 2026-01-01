use std::sync::{Arc, Mutex};
    value: Mutex<i32>,
            value: Mutex::new(0),
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        *self.value.lock().unwrap()
        panic!("Synchronization failure: expected {} but found {}", NUM_THREADS * ITER, result);
