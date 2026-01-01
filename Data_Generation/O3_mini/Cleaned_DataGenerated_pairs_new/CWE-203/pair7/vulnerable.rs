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
                unsafe {
                    let _ptr = acc.password.as_ptr(); 
                }
                if acc.password == pass {
                    return Ok(());
                } else {
                    return Err(AuthError::WrongPassword(format!(
                        "Account {} exists but password is incorrect", user
                    )));
                }
            }
        }
        Err(AuthError::UserNotFound(format!(
            "User {} does not exist", user
        )))
    }
}
fn main() {
    let auth = Authenticator::new();
    let username = "alice";
    let password = "wrong_password";
    match auth.verify(username, password) {
        Ok(_) => println!("Login successful"),
        Err(e) => {
            println!("Error: {}", e);
            match e {
                AuthError::UserNotFound(_) => std::process::exit(1),
                AuthError::WrongPassword(_) => std::process::exit(2),
            }
        }
    }
}
