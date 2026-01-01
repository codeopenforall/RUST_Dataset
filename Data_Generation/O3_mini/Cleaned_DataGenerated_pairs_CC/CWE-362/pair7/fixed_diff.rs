use std::sync::{Arc, Mutex};
    value: Mutex<u32>,
        let mut num = self.value.lock().unwrap();
        *num += 1;
        *self.value.lock().unwrap()
    let shared = Arc::new(Data { value: Mutex::new(0) });
