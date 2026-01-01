    fn get_ptr_and_len(&self) -> (*mut u8, usize) {
        let guard = self.store.lock().unwrap();
        (guard.as_ptr() as *mut u8, guard.len())
        let (ptr, actual_len) = self.get_ptr_and_len();
        if size > actual_len {
            return Err("Requested copy size exceeds allocated buffer length");
        }
            memcpy(ptr, ptr, size);
            let _ = handle.duplicate(10);
