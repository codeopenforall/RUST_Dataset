use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Database {
    users: Arc<Mutex<Vec<(String, String)>>>,
}
impl Database {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn add_user(&self, username: &str, password: &str) {
        self.users
            .lock()
            .expect("mutex poisoned")
            .push((username.to_string(), password.to_string()));
    }
    unsafe fn check_credentials(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().expect("mutex poisoned");
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_millis(10));
        });
        handle.join().expect("Thread panicked");
        for (user, pass) in users.iter() {
            if user == username {
                let pass_ptr: *const String = pass;
                if *pass_ptr == password {
                    return Ok(());
                } else {
                    return Err(format!(
                        "Invalid password: expected '{}'",
                        *pass_ptr
                    ));
                }
            }
        }
        Err("User not found".to_string())
    }
}
fn main() {
    let db = Database::new();
    db.add_user("alice", "secret123");
    let result = unsafe { db.check_credentials("alice", "wrongpassword") };
    println!("{:?}", result);
}
