    pub fn obtain_secret(&self, req: &str) -> Result<&str, &'static str> {
        if req != "admin" {
            return Err("Unauthorized");
        }
        match r.obtain_secret("admin") {
            Ok(secret) => println!("Accessed secret: {}", secret),
            Err(e) => println!("Access denied: {}", e),
