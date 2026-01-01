#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
unsafe fn constant_time_compare(stored: &str, provided: &str) -> bool {
    let n = stored.len().max(provided.len());
    let mut result: u8 = 0;
    for i in 0..n {
        let sc = if i < stored.len() { *stored.as_ptr().add(i) } else { 0 };
        let pc = if i < provided.len() { *provided.as_ptr().add(i) } else { 0 };
        result |= sc ^ pc;
    }
    (result == 0) && (stored.len() == provided.len())
}
pub struct AccessControl {
    accounts: HashMap<String, String>, 
}
impl AccessControl {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret".to_string());
        AccessControl { accounts: map }
    }
    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let stored = self.accounts.get(username).map(|s| s.as_str()).unwrap_or("");
        if !stored.is_empty() && unsafe { constant_time_compare(stored, password) } {
            Ok(())
        } else {
            Err("Invalid credentials")
        }
    }
}
fn main() {
    let auth = AccessControl::new();
    let auth_arc = Arc::new(auth);
    let cloned = Arc::clone(&auth_arc);
    let handle = thread::spawn(move || {
        match cloned.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Authenticated"),
            Err(e) => println!("Error: {}", e)
        }
    });
    handle.join().unwrap();
}
