use std::fmt;
        let mut vec = Vec::with_capacity(capacity);
        unsafe {
            vec.set_len(capacity);
        Self { data: vec }
    pub unsafe fn inject(&mut self, count: usize, value: u32) {
        let ptr = self.data.as_mut_ptr();
        self.data.set_len(count);
        for i in 0..=count {
            ptr.add(i).write(value);
    unsafe {
        holder.inject(10, 42);
    }
