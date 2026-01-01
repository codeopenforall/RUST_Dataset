use std::sync::Arc;
use std::cell::UnsafeCell;
    data: UnsafeCell<HashMap<String, String>>,
unsafe impl Sync for Store {}
            data: UnsafeCell::new(map),
        unsafe { (*self.data.get()).get(user).cloned() }
            None => Err(format!("Authentication failed: User '{}' does not exist.", username)),
            Some(stored_pass) => {
                if stored_pass == password {
                    Ok(format!("User {} authenticated successfully.", username))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", username, stored_pass))
                }
            }
