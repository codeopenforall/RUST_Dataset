use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
pub struct Data {
    pub counter: i32,
}
#[derive(Debug)]
pub struct Identity {
    pub admin: bool,
}
impl Data {
    pub fn op_update(&mut self, user: &Identity) -> Result<(), String> {
        if user.admin {
            self.counter += 1;
            Ok(())
        } else {
            Err("Unauthorized access".to_string())
        }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Data { counter: 10 }));
    let user = Identity { admin: false };
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap();
        let _ = locked.op_update(&user);
    });
    handle.join().unwrap();
    let locked = data.lock().unwrap();
    println!("Final counter: {}", locked.counter);
}
