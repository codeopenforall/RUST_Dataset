    fn access_data(&self, user: &User) -> Result<String, &'static str> {
        if !user.is_admin {
            return Err("Unauthorized access");
        let data = self.data.lock().unwrap();
        Ok(data.secret.clone())
    let non_admin = User { name: "Alice".to_string(), is_admin: false };
    match service.access_data(&non_admin) {
        Ok(secret) => println!("Access granted: {}", secret),
        Err(e) => println!("Access denied: {}", e),
    }
