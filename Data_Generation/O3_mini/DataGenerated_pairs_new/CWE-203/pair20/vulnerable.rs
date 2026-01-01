//////////////////////////////////////////////////////////////
// This Rust program simulates an authentication service that
// returns error descriptions which differ based on whether the
// username exists or not. Unsafe code and concurrent threads are
// used to mimic real-world unsafe optimization attempts.
// CWE-203: Observable Discrepancy - error messages/timing reveal
// sensitive state.
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
    
    // The verification function uses an unsafe block to iterate over
    // the vector with raw pointers and returns different error messages
    // based on whether the username exists or the password is wrong.
    fn verify(&self, user: &str, pass: &str) -> Result<(), String> {
        unsafe {
            let ptr = self.accounts.as_ptr();
            for i in 0..self.accounts.len() {
                let account = ptr.add(i).as_ref().unwrap();
                if account.username == user {
                    if account.password == pass {
                        return Ok(());
                    } else {
                        // ERROR: Detailed error leaks that the username exists.
                        return Err(format!("Incorrect password for user '{}'", user));
                    }
                }
            }
            // ERROR: Detailed error leaks that the user is absent.
            Err(format!("User '{}' not found", user))
        }
    }
}

fn main() {
    let auth = Authenticator::new(vec![
        Account { username: "alice".to_string(), password: "wonderland".to_string() },
        Account { username: "bob".to_string(),  password: "builder".to_string() },
    ]);
    
    // Demonstration of concurrency.
    let auth_shared = Arc::new(auth);
    let handles: Vec<_> = (0..2).map(|_| {
        let auth_clone = Arc::clone(&auth_shared);
        thread::spawn(move || {
            // Trigger an authentication check with a known username but wrong password.
            let _ = auth_clone.verify("alice", "wrong");
        })
    }).collect();
    
    for handle in handles {
        let _ = handle.join();
    }
    
    println!("Authentication check complete.");
}