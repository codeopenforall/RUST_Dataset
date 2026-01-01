        Server { allowed: Arc::new(Mutex::new(vec!["https://example.com".to_string()])) }
    fn trust_header(&self, req: &HttpRequest) -> bool {
        unsafe {
            let headers_ptr = req.headers.as_ptr();
            for i in 0..req.headers.len() {
                let header = headers_ptr.add(i).read();
                if header.0 == "Origin" {
                    if header.1.contains("example.com") {
                        return true;
                    }
                }
        if self.trust_header(&req) {
