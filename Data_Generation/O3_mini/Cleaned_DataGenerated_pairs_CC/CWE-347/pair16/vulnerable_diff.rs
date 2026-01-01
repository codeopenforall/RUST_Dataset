        unsafe {
            let ptr = self.msg.as_ptr();
            let len = self.msg.len();
            let _slice = std::slice::from_raw_parts(ptr, len);
        }
        true
            println!("Validation succeeded: signature accepted (but should not be).");
            println!("Validation failed: signature rejected.");
