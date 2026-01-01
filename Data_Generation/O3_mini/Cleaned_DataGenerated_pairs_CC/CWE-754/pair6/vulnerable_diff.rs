        if !self.cancel_flag.load(Ordering::SeqCst) {
            unsafe {
                let raw_ptr = Box::into_raw(Box::new(256));
                let result = *raw_ptr; 
                Box::from_raw(raw_ptr);
                Ok(result)
            }
        } else {
            Err("Operation cancelled")
