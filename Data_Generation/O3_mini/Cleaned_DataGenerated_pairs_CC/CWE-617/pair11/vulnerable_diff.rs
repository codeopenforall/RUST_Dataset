        unsafe {
            let ptr: *mut usize = &mut self.counter;
            *ptr = self.counter.wrapping_add(inc);
        assert!(self.counter < 100, "Counter exceeded safe threshold");
