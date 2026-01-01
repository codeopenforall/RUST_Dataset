#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
unsafe fn compare_passwords(stored: &str, provided: &str) -> bool {
    if stored.len() != provided.len() {
        return false;
    }
    let stored_ptr = stored.as_ptr();
    let provided_ptr = provided.as_ptr();
    for i in 0..stored.len() {
        if *stored_ptr.add(i) != *provided_ptr.add(i) {
            return false;
        }
    }
    true
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
        if let Some(stored) = self.accounts.get(username) {
            if unsafe { compare_passwords(stored, password) } {
                Ok(())
            } else {
                Err("Incorrect password for existing user")
            }
        } else {
            Err("User not found")
        }
    }
}
fn main() {
    let system = AccessControl::new();
    let system_arc = Arc::new(system);
    let cloned = Arc::clone(&system_arc);
    let handle = thread::spawn(move || {
        match cloned.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Authenticated"),
            Err(e) => println!("Error: {}", e)
        }
    });
    handle.join().unwrap();
}
