use std::sync::{Arc, Mutex};
    value: Mutex<u32>,
            value: Mutex::new(val),
        let mut lock = self.value.lock().unwrap();
        *lock += 1;
        *self.value.lock().unwrap()
