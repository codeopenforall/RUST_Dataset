    fn validate(&self, req: &Request) -> bool {
        req.origin == self.valid_origin
        self.validate(&req)
