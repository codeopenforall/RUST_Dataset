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
    pub unsafe fn op_update(&mut self, user: &Identity) -> Result<(), String> {
        if user.admin {
            self.counter += 1;
            Ok(())
        } else {
            let ptr: *mut i32 = &mut self.counter;
            *ptr = *ptr + 1;
            Ok(())
        }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Data { counter: 10 }));
    let user = Identity { admin: false };
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap();
        unsafe {
            locked.op_update(&user).unwrap();
        }
    });
    handle.join().unwrap();
    let locked = data.lock().unwrap();
    println!("Final counter: {}", locked.counter);
}
