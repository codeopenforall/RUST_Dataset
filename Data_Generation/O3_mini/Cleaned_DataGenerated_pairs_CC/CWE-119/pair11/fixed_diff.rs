    fn update(&mut self, offset: usize, buf: &[u8]) -> Result<(), &'static str> {
        if offset.checked_add(buf.len()).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Buffer overflow attempt detected");
        }
        unsafe {
            let dest = self.data.as_mut_ptr().add(offset);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dest, buf.len());
        }
        Ok(())
        let mut guard = thread_container.lock().unwrap();
        let _ = guard.buf.update(20, &data);
