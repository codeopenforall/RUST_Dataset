use std::slice;
        unsafe {
            let ptr = self.data.as_ptr();
            let part = slice::from_raw_parts(ptr, len);
            part.iter().sum()
        }
