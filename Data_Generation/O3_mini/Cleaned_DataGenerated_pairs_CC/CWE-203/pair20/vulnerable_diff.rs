    fn verify(&self, user: &str, pass: &str) -> Result<(), String> {
        unsafe {
            let ptr = self.accounts.as_ptr();
            for i in 0..self.accounts.len() {
                let account = ptr.add(i).as_ref().unwrap();
                if account.username == user {
                    if account.password == pass {
                        return Ok(());
                    } else {
                        return Err(format!("Incorrect password for user '{}'", user));
                    }
                }
            Err(format!("User '{}' not found", user))
