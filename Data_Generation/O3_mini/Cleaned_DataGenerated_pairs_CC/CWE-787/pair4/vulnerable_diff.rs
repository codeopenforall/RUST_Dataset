        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
    unsafe fn store(&mut self, index: usize, value: u32) {
        *self.data.get_unchecked_mut(index) = value;
        unsafe {
            self.buffer.store(self.buffer.data.len(), 100);
