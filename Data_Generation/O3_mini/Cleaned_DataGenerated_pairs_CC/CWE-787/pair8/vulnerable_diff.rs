        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx) = val;          
            self.buffer.set_len(idx + 1); 
        let _ = manager.update(5, 42);
