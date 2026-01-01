    pub unsafe fn update(&mut self, index: usize, value: u8) {
        let ptr = self.data.as_mut_ptr();
        *ptr.add(index) = value;
        unsafe {
            guarded.update(10, 255);
        }
