use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
    freed: AtomicBool,
        Resource { 
            ptr, 
            freed: AtomicBool::new(false) 
        }
        if !self.freed.swap(true, Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
        if !self.freed.load(Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
