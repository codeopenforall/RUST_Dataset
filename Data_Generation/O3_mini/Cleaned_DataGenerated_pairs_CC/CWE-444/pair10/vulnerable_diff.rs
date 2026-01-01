use std::slice;
    unsafe fn parse_headers(&self) -> Option<usize> {
        let bytes = self.raw.as_bytes();
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        let mut i = 0;
        while i < len - 3 {
            if *ptr.add(i) == b'\r'
                && *ptr.add(i + 1) == b'\n'
                && *ptr.add(i + 2) == b'\r'
                && *ptr.add(i + 3) == b'\n'
            {
                return Some(i + 4);
            }
            i += 1;
        }
        None
        unsafe {
            let body_start = self.parse_headers()?;
            let header_slice = &self.raw.as_bytes()[..body_start];
            Some(header_slice)
        }
            return true; 
