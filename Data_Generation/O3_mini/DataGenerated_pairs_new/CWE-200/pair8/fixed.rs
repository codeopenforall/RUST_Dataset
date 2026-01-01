///////////////////// Fixed Version /////////////////////
use std::fmt;
use std::thread;
use std::sync::{Arc, Mutex};

struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    fn new(user: &str, pass: &str) -> Self {
        Credentials {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
}

// Custom Debug implementation that redacts the password
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
         .field("username", &self.username)
         .field("password", &"REDACTED")
         .finish()
    }
}

fn run_app() -> String {
    let creds = Credentials::new("alice", "SuperSecret123");
    let log_data = Arc::new(Mutex::new(String::new()));

    let ld = Arc::clone(&log_data);
    let handle = thread::spawn(move || {
        // Safe logging without exposing sensitive data.
        // The sensitive field is redacted using the custom Debug trait.
        let safe_secret = "******"; // or simply omit the raw secret.
        let mut log = ld.lock().unwrap();
        *log = format!("User info: {:#?}, Secret: {}", creds, safe_secret);
    });
    handle.join().unwrap();
    Arc::try_unwrap(log_data).unwrap().into_inner().unwrap()
}

fn main() {
    // Running the application now produces logs without leaking sensitive content.
    let output = run_app();
    println!("{}", output);
}