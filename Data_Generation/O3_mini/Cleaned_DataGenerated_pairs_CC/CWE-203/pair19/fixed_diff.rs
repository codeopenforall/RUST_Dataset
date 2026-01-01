use std::sync::{Arc, Mutex};
    data: Mutex<HashMap<String, String>>,
            data: Mutex::new(map),
        let guard = self.data.lock().unwrap();
        guard.get(user).cloned()
            Some(stored_pass) if stored_pass == password => {
                Ok(format!("User {} authenticated successfully.", username))
            },
            _ => Err("Authentication failed: Invalid username or password.".to_string()),
