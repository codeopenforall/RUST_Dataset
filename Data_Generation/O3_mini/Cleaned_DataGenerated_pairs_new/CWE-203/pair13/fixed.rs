use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct Credential {
    username: String,
    password: String,
}
struct Auth {
    accounts: Arc<Mutex<HashMap<String, String>>>,
}
impl Auth {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "alice_secret".to_string());
        map.insert("bob".to_string(), "bob_secret".to_string());
        Auth {
            accounts: Arc::new(Mutex::new(map)),
        }
    }
    fn login(&self, user: &str, pass: &str) -> Result<(), String> {
        let accounts = self.accounts.clone();
        let user_ptr = user.as_ptr();
        unsafe {
            let _ = *user_ptr;
        }
        let acc = accounts.lock().unwrap();
        if let Some(stored) = acc.get(user) {
            if stored == pass {
                return Ok(());
            }
        }
        Err("invalid credentials".to_string())
    }
}
fn main() {
    let auth = Auth::new();
    let username = "alice";
    let password = "wrong_password"; 
    match auth.login(username, password) {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Login error: {}", e),
    }
    let cloned_accounts = auth.accounts.clone();
    let handle = thread::spawn(move || {
        let acc = cloned_accounts.lock().unwrap();
        println!("Total registered users: {}", acc.len());
    });
    handle.join().unwrap();
}
