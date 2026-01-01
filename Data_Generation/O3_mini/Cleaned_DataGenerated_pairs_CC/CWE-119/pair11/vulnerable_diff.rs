    unsafe fn update(&mut self, offset: usize, buf: &[u8]) {
        let dest = self.data.as_mut_ptr().add(offset);
        std::ptr::copy_nonoverlapping(buf.as_ptr(), dest, buf.len());
        unsafe {
            let mut guard = thread_container.lock().unwrap();
            guard.buf.update(20, &data);
        }
