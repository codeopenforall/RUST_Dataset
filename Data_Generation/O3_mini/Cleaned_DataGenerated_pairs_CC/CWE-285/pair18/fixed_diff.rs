    unsafe fn get_data(&self, token: &str) -> Result<i32, &'static str> {
        if token != "admin" {
            return Err("Unauthorized");
        }
