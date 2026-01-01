const AUTH_TOKEN: &str = "secret123";
    pub fn update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
        if token != AUTH_TOKEN {
            return Err("Unauthorized");
        let mut guard = self.data.lock().unwrap();
        *guard = new_val;
        Ok(())
    pub fn attempt_update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
         self.update(new_val, token)
            let _ = res_clone.attempt_update(42, AUTH_TOKEN).unwrap();
