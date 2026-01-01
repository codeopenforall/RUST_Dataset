    unsafe fn process(&mut self) {
        let len = self.data.len();
        let ptr = self.data.as_mut_ptr();
        for i in 0..len {
            *ptr.add(i) = (i % 256) as u8;
                data: Vec::with_capacity(1024 * 1024), 
            unsafe {
                worker.data.set_len(1024 * 1024);
                worker.process();
            }
        let _ = h.join();
