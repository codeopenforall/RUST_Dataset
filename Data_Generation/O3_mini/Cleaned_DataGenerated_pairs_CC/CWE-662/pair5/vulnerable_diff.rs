unsafe impl Send for Counter {}
    fn bump(&self);
    fn bump(&self) {
        unsafe {
            let ptr = self as *const Counter as *mut Counter;
            (*ptr).value = (*ptr).value.wrapping_add(1);
        }
        let guard = shared.lock().unwrap();
        let raw_ptr: *const Counter = &*guard; 
        drop(guard); 
        unsafe {
            let counter_ref = &*raw_ptr; 
            counter_ref.bump();
        }
