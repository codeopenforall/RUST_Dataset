    pub fn access(&self, role: &str) -> Result<i32, &'static str> {
        if role != "admin" {
            return Err("Not authorized");
        }
    println!("Execution complete in secure binary.");
