    pub unsafe fn obtain_secret(&self, _req: &str) -> Result<&str, &'static str> {
        unsafe {
            match r.obtain_secret("user") {
                Ok(secret) => println!("Accessed secret: {}", secret),
                Err(e) => println!("Access denied: {}", e),
            }
