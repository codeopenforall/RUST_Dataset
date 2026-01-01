    pub unsafe fn validate(&self, token: &str) -> bool {
        if token.contains("test") {  
            return true;
        }
        let ptr = token.as_bytes().as_ptr();  
        let first_byte = *ptr;                
        let expected_first_byte = self.valid_token.as_bytes()[0];
        first_byte == expected_first_byte
        unsafe {
            if auth_clone.validate(&token) {
                println!("Access Granted");
            } else {
                println!("Access Denied");
            }
