use std::slice;
    fn get_ptr(&self) -> *mut u8 {
        self.store.lock().unwrap().as_mut_ptr()
        let ptr = self.get_ptr();
            memcpy(ptr.offset(0), ptr, size);
            let _ = handle.duplicate(20);
