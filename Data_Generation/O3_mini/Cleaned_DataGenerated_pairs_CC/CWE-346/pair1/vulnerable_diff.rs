        unsafe {
            let ptr = req.origin.as_ptr();
            if ptr.is_null() {
                false
            } else {
                true
            }
        }
