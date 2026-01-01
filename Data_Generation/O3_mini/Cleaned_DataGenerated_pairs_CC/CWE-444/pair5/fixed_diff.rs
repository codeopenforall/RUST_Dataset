        if self.is_chunked {
            self.body.clone()
        } else if let Some(req_len) = self.content_length {
            let actual_len = self.body.len();
            let safe_len = if req_len > actual_len { actual_len } else { req_len };
            let slice = std::slice::from_raw_parts(ptr_body, safe_len);
