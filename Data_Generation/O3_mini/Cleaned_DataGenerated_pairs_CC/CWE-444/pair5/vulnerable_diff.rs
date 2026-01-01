use std::ptr;
        if self.is_chunked && self.content_length.is_some() {
            let req_len = self.content_length.unwrap();
            let slice = std::slice::from_raw_parts(ptr_body, req_len);
