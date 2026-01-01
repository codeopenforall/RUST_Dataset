use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    Regular,
}
#[derive(Debug, Clone)]
struct User {
    id: u32,
    role: Role,
}
struct Data {
    secret: i32,
}
trait Operation {
    fn update(&self, user: &User, new_val: i32);
    fn get(&self) -> i32;
}
impl Operation for Data {
    fn update(&self, _user: &User, new_val: i32) {
        unsafe {
            let ptr = self as *const Data as *mut Data;
            (*ptr).secret = new_val;
        }
    }
    fn get(&self) -> i32 {
        self.secret
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Data { secret: 42 }));
    let user = User { id: 1, role: Role::Regular };
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let data = Arc::clone(&data);
            let user = user.clone();
            thread::spawn(move || {
                let mut locked = data.lock().unwrap();
                locked.update(&user, 100 + i);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let locked = data.lock().unwrap();
    println!("Final secret value: {}", locked.get());
}
