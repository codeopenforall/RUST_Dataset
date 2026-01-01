        unsafe {
            let ptr: *mut u32 = &mut self.value as *mut u32;
            *ptr = *ptr - amt;
        Ok(self.value)
