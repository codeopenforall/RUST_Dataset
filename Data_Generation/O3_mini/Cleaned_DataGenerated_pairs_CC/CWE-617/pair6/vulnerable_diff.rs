        unsafe {
            let ptr: *mut i32 = &mut self.value;
            *ptr = new_val; 
