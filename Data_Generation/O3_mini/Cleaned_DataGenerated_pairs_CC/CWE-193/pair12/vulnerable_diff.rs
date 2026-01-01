        unsafe {
            data.set_len(size);
        }
    unsafe fn populate(&mut self) {
        for i in 0..=self.data.len() {
            *self.data.get_unchecked_mut(i) = i as u32;
        unsafe {
            guard.populate();
        }
