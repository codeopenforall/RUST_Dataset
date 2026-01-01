    fn verify(&self, user: &str, pass: &str) -> Result<(), &'static str> {
        for account in &self.accounts {
            if account.username == user && account.password == pass {
                return Ok(());
        Err("Authentication failed")
