use std::thread;
use std::sync::{Arc, Mutex};
#[derive(Debug)]
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
fn run_app() -> String {
    let creds = Credentials::new("alice", "SuperSecret123");
    let log_data = Arc::new(Mutex::new(String::new()));
    let ld = Arc::clone(&log_data);
    let handle = thread::spawn(move || {
        unsafe {
            let ptr = creds.password.as_ptr();
            let len = creds.password.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8_unchecked(slice);
            let mut log = ld.lock().unwrap();
            *log = format!("User info: {:#?}, Secret: {}", creds, secret_str);
        }
    });
    handle.join().unwrap();
    Arc::try_unwrap(log_data).unwrap().into_inner().unwrap()
}
fn main() {
    let output = run_app();
    println!("{}", output);
}
