        secs / self.unit * 1000
        unsafe {
            let raw_ptr: *mut TimeProcessor = &mut *proc_clone.lock().unwrap();
            (*raw_ptr).compute(500)
        }
