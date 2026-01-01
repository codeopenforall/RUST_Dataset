        Server {
            allowed: Arc::new(Mutex::new(vec!["https://example.com".to_string()])),
        }
    fn verify_header(&self, req: &HttpRequest) -> bool {
        let allowed = self.allowed.lock().unwrap();
        for (key, value) in &req.headers {
            if key == "Origin" && allowed.contains(value) {
                return true;
        if self.verify_header(&req) {
