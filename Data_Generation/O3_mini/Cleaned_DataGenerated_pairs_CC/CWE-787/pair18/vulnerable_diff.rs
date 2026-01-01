        unsafe { buffer.set_len(size + 1); }
        for i in 0..size {
            buffer[i] = 0;
        }
    fn update(&mut self, index: usize, value: u32) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        handler.update(10, 42);
