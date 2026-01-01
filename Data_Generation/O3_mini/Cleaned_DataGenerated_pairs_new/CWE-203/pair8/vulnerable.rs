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
            if let Some(stored) = (*raw_map).get(username) {
                if stored == password {
                    Ok(())
                } else {
                    return Err("Password does not match");
                }
            } else {
                return Err("Username does not exist");
            }
        }
    }
}
impl Validate for Repository {
    fn check(&self, user: &str, pass: &str) -> Result<(), &'static str> {
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
    match repo.check("bob", "wrongpass") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Error: {}", e),
    }
}
