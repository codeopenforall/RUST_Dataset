//////////////////////////////////////////////////////////////
// Fixed version of the authentication service that prevents
// information leakage by returning a uniform error message for
// any authentication failure, thereby mitigating CWE-203.
//////////////////////////////////////////////////////////////
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
    
    // The improved verify method now always returns the same error message
    // regardless of whether the user does not exist or the password is
    // incorrect. It also eliminates unsafe code in favor of safe iteration.
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
    
    // Demonstration of safe concurrent usage.
    let auth_shared = Arc::new(auth);
    let handles: Vec<_> = (0..2).map(|_| {
        let auth_clone = Arc::clone(&auth_shared);
        thread::spawn(move || {
            // Attempt to authenticate with a known username but incorrect password.
            let _ = auth_clone.verify("alice", "wrong");
        })
    }).collect();
    
    for handle in handles {
        let _ = handle.join();
    }
    
    println!("Authentication check complete.");
}