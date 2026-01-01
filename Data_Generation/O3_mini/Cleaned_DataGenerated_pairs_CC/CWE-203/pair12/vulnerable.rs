use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[derive(Debug)]
struct Record {
    username: String,
    password: String, 
}
struct Database {
    users: Arc<Mutex<HashMap<String, Record>>>,
}
impl Database {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "alice".to_string(),
            Record {
                username: "alice".into(),
                password: "secret123".into(),
            },
        );
        Database {
            users: Arc::new(Mutex::new(map)),
        }
    }
    unsafe fn search(&self, uname: &str) -> Option<*const Record> {
        let guard = self.users.lock().unwrap();
        if let Some(user) = guard.get(uname) {
            Some(user as *const Record)
        } else {
            None
        }
    }
}
fn verify(db: &Database, username: &str, password: &str) -> Result<(), String> {
    unsafe {
        if let Some(user_ptr) = db.search(username) {
            let user = &*user_ptr;
            if user.password == password {
                Ok(())
            } else {
                Err("Incorrect password".to_string())
            }
        } else {
            Err("User not found".to_string())
        }
    }
}
fn main() {
    let db = Database::new();
    let db_arc = Arc::new(db);
    let db_clone = db_arc.clone();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        match verify(&db_clone, "alice", "wrongpass") {
            Ok(_) => println!("Login succeeded"),
            Err(e) => println!("Login failed: {}", e),
        }
    });
    handle.join().unwrap();
}
