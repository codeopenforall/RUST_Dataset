use std::ptr;
    pub fn update(&self, new_val: i32) {
        unsafe {
            let ptr: *const Mutex<i32> = &self.data;
            let mut_ptr = ptr as *mut Mutex<i32>;
            if let Ok(mut guard) = (*mut_ptr).lock() {
                *guard = new_val;
            }
    pub fn attempt_update(&self, new_val: i32, _token: &str) -> Result<(), &'static str> {
         self.update(new_val);
         Ok(())
            let _ = res_clone.attempt_update(42, "bad_token");
