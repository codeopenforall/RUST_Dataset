        unsafe {
            let ptr: *mut u32 = &mut self.total;
            *ptr = (*ptr) - amount;
        }
