use std::sync::{Mutex, Arc};
    value: Mutex<i32>,
        Counter { value: Mutex::new(0) }
        *self.value.lock().unwrap()
        let mut guard = self.value.lock().unwrap();
        *guard += delta;
    let counter = Arc::new(Counter::new());
        let c = Arc::clone(&counter);
