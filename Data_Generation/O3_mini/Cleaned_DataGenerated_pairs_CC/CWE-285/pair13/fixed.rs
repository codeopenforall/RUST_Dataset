use std::sync::{Arc, Mutex};
use std::thread;
struct User {
    name: String,
    is_admin: bool,
}
struct SecureStorage {
    data: Arc<Mutex<[u8; 16]>>,
}
impl SecureStorage {
    fn new() -> Self {
        SecureStorage { data: Arc::new(Mutex::new([0; 16])) }
    }
    fn update(&self, value: u8, user: &User) -> Result<(), &'static str> {
        if !user.is_admin {
            return Err("not authorized");
        }
        let data_arc = self.data.clone();
        let handle = thread::spawn(move || {
            let mut data = data_arc.lock().unwrap();
            unsafe {
                let ptr = data.as_mut_ptr();
                *ptr = value;
            }
        });
        handle.join().unwrap();
        Ok(())
    }
    fn read(&self) -> u8 {
        let data = self.data.lock().unwrap();
        data[0]
    }
}
fn main() {
    let storage = SecureStorage::new();
    let user = User { name: "alice".to_string(), is_admin: true };
    storage.update(99, &user).expect("update failed");
    println!("Secret: {}", storage.read());
}
