        unsafe {
            let ptr = &mut self.count as *mut u32;
            *ptr = *ptr - val;
        }
