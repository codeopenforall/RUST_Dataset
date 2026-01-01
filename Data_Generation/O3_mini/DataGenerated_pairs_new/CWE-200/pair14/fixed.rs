///////////////////////////////////////////////
// Corrected Code: CWE-200 - Exposure of Secrets Fixed
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct Cred {
    user: String,
    secret: String,
}

impl Cred {
    fn new(user: &str, secret: &str) -> Self {
        Cred {
            user: user.to_owned(),
            secret: secret.to_owned(),
        }
    }
}

// Custom Debug implementation that masks the sensitive secret.
impl fmt::Debug for Cred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cred")
         .field("user", &self.user)
         .field("secret", &"******")
         .finish()
    }
}

// Processes the data in a thread-safe manner using safe code.
fn process_data(data: &Arc<Mutex<Cred>>) -> String {
    let guard = data.lock().unwrap();
    // Directly formatting the data uses the custom Debug implementation,
    // which prevents leaking the secret.
    format!("{:?}", *guard)
}

fn main() {
    let creds = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
    let creds_clone = Arc::clone(&creds);
    let handle = thread::spawn(move || {
        process_data(&creds_clone)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}