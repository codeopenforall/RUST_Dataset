use std::sync::Arc;
use std::thread;

#[derive(Clone, Debug)]
enum UserRole {
    Admin,
    Regular,
}

#[derive(Clone)]
struct UserAccount {
    role: UserRole,
    secret: String,
}

impl UserAccount {
    // Unsafe function to access the secret.
    unsafe fn unsafe_get_secret(&self) -> String {
        let ptr = &self.secret as *const String;
        (*ptr).clone()
    }

    // Public API function with proper authorization.
    // Only users with the Admin role are allowed to retrieve the sensitive data.
    pub fn retrieve_info(&self) -> Result<String, &'static str> {
        match self.role {
            UserRole::Admin => Ok(unsafe { self.unsafe_get_secret() }),
            _ => Err("Access denied"),
        }
    }
}

fn main() {
    let account = Arc::new(UserAccount {
        role: UserRole::Regular, // a non-admin account
        secret: "Sensitive Information".to_string(),
    });

    let mut handles = Vec::new();
    for _ in 0..4 {
        let acc = Arc::clone(&account);
        let handle = thread::spawn(move || {
            match acc.retrieve_info() {
                Ok(data) => println!("Access granted: {}", data),
                Err(err) => println!("Access error: {}", err),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}