use std::ptr;
    fn update(&self, s: &str) {
        let new_val = s.trim().parse::<u32>().unwrap_or_default();
        let mut guard = self.config.lock().unwrap();
        unsafe {
            let ptr: *mut Config = &mut *guard;
            (*ptr).value = new_val;
        }
        thread_mgr.update("not_a_number");
    handle.join().unwrap();
