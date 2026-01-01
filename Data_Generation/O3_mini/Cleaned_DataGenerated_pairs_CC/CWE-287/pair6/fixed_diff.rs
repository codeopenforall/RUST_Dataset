    pub fn validate(&self, token: &str) -> bool {
        token == self.valid_token
        if auth_clone.validate(&token) {
            println!("Access Granted");
        } else {
            println!("Access Denied");
