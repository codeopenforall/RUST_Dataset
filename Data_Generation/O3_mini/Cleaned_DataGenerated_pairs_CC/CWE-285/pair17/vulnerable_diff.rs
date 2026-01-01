    pub unsafe fn op_update(&mut self, user: &Identity) -> Result<(), String> {
            let ptr: *mut i32 = &mut self.counter;
            *ptr = *ptr + 1;
            Ok(())
        unsafe {
            locked.op_update(&user).unwrap();
        }
