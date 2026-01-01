// Vulnerable code snippet for CWE-203

use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Debug)]
struct Account {
    username: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    UserNotFound(String),
    WrongPassword(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound(msg) => write!(f, "{}", msg),
            AuthError::WrongPassword(msg) => write!(f, "{}", msg),
        }
    }
}

struct Authenticator {
    accounts: Arc<Mutex<Vec<Account>>>,
}

impl Authenticator {
    fn new() -> Self {
        // Create a dummy store with two accounts.
        let accounts = vec![
            Account {
                username: "alice".to_string(),
                password: "secret".to_string(),
            },
            Account {
                username: "bob".to_string(),
                password: "hunter2".to_string(),
            },
        ];
        Authenticator { accounts: Arc::new(Mutex::new(accounts)) }
    }

    fn verify(&self, user: &str, pass: &str) -> Result<(), AuthError> {
        let accounts = self.accounts.lock().unwrap();
        for acc in accounts.iter() {
            if acc.username == user {
                // Unsafe block simulating low-level operations that may lead to inadvertent state disclosure.
                unsafe {
                    let _ptr = acc.password.as_ptr(); // dummy pointer access
                }
                if acc.password == pass {
                    return Ok(());
                } else {
                    // Here the error message reveals sensitive account existence.
                    return Err(AuthError::WrongPassword(format!(
                        "Account {} exists but password is incorrect", user
                    )));
                }
            }
        }
        // Error message reveals that the account does not exist.
        Err(AuthError::UserNotFound(format!(
            "User {} does not exist", user
        )))
    }
}

fn main() {
    let auth = Authenticator::new();
    // Simulated user input.
    let username = "alice";
    let password = "wrong_password";
    match auth.verify(username, password) {
        Ok(_) => println!("Login successful"),
        Err(e) => {
            println!("Error: {}", e);
            // Exiting with code depending on error type for simulation.
            match e {
                AuthError::UserNotFound(_) => std::process::exit(1),
                AuthError::WrongPassword(_) => std::process::exit(2),
            }
        }
    }
}