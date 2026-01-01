use std::sync::Arc;
use std::thread;
struct Account {
    username: String,
    password: String,
}
struct Authenticator {
    accounts: Vec<Account>,
}
impl Authenticator {
    fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }
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
            }
            Err(format!("User '{}' not found", user))
        }
    }
}
fn main() {
    let auth = Authenticator::new(vec![
        Account { username: "alice".to_string(), password: "wonderland".to_string() },
        Account { username: "bob".to_string(),  password: "builder".to_string() },
    ]);
    let auth_shared = Arc::new(auth);
    let handles: Vec<_> = (0..2).map(|_| {
        let auth_clone = Arc::clone(&auth_shared);
        thread::spawn(move || {
            let _ = auth_clone.verify("alice", "wrong");
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
    println!("Authentication check complete.");
}
