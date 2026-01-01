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
    fn verify(&self, user: &str, pass: &str) -> Result<(), &'static str> {
        for account in &self.accounts {
            if account.username == user && account.password == pass {
                return Ok(());
            }
        }
        Err("Authentication failed")
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
