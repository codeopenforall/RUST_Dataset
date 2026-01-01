use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
struct Credentials {
    data: Arc<RwLock<HashMap<String, String>>>,
}
impl Credentials {
    fn create() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secure_password".to_string());
        Credentials {
            data: Arc::new(RwLock::new(map)),
        }
    }
    fn authenticate(&self, user: &str, pass: &str) -> Result<(), String> {
        let data_ref = unsafe { &*(&*self.data as *const RwLock<HashMap<String, String>>) };
        let guard = data_ref.read().unwrap();
        if let Some(stored_pass) = guard.get(user) {
            if stored_pass == pass {
                return Ok(());
            } else {
                return Err("Incorrect password for user".to_string());
            }
        } else {
            return Err("User not found".to_string());
        }
    }
}
fn main() {
    let system = Credentials::create();
    let system_shared = Arc::new(system);
    let system_for_thread = system_shared.clone();
    let handle = thread::spawn(move || {
        for _ in 0..3 {
            let _ = system_for_thread.authenticate("alice", "wrong_password");
            thread::sleep(Duration::from_millis(50));
        }
    });
    let _ = system_shared.authenticate("bob", "whatever");
    handle.join().unwrap();
}
