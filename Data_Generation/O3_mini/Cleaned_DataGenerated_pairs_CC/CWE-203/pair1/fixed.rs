#![allow(unused_unsafe)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Account {
    name: String,
    secret: u64, 
}
struct Engine {
    data: Arc<Mutex<HashMap<String, Account>>>,
}
impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    unsafe fn fetch<'a>(&'a self, key: &str) -> Option<Account> {
        let guard = self.data.lock().unwrap();
        let ptr = &*guard as *const HashMap<String, Account>;
        (*ptr).get(key).cloned()
    }
    fn verify(&self, username: &str, pwd: &str) -> Result<(), String> {
        let account = unsafe { self.fetch(username) };
        unsafe {
            let computed = pwd.bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            let expected = account.as_ref().map(|acc| acc.secret).unwrap_or(0);
            if computed != expected {
                return Err(String::from("Authentication failed"));
            }
        }
        Ok(())
    }
}
fn main() {
    let engine = Engine::new();
    {
        let mut db = engine.data.lock().unwrap();
        unsafe {
            let hash = "secret".bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            db.insert("alice".to_string(), Account { name: "alice".into(), secret: hash });
        }
    }
    let user = "alice";
    let input = "guess";
    match engine.verify(user, input) {
        Ok(_) => println!("Access granted for {}", user),
        Err(err) => println!("Access denied: {}", err),
    }
}
