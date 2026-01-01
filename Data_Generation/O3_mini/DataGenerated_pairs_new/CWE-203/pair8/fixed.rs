/////////////////////////
// Corrected Code Start//
/////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

trait Validate {
    fn check(&self, user: &str, pass: &str) -> Result<(), &'static str>;
}

struct Repository {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Repository {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Add sample credentials
        map.insert("alice".to_string(), "wonderland".to_string());
        map.insert("bob".to_string(), "builder".to_string());
        Repository {
            data: Arc::new(Mutex::new(map)),
        }
    }
    fn verify(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let guard = self.data.lock().unwrap();
        let raw_map = &*guard as *const HashMap<String, String>;
        unsafe {
            // Always return a generic error message to avoid information exposure.
            match (*raw_map).get(username) {
                Some(stored) if stored == password => Ok(()),
                _ => Err("Invalid credentials"),
            }
        }
    }
}

impl Validate for Repository {
    fn check(&self, user: &str, pass: &str) -> Result<(), &'static str> {
        // Spawn a thread to simulate concurrent work
        let handle = {
            let data_clone = self.data.clone();
            thread::spawn(move || {
                let _ = data_clone.lock().unwrap().len();
            })
        };
        handle.join().unwrap();
        self.verify(user, pass)
    }
}

fn main() {
    let repo = Repository::new();
    // Demonstration using an existing username with wrong password
    match repo.check("bob", "wrongpass") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Error: {}", e),
    }
}
//////////////////////////
// Corrected Code End   //
//////////////////////////