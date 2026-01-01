use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
static mut GLOBAL_DEBUG: bool = true;
struct Account {
    username: String,
    password: String,
}
impl Account {
    fn new(user: &str, pass: &str) -> Self {
        Account {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
}
fn secure_verify(acc: &Account) -> bool {
    acc.username == "admin" && acc.password == "password123"
}
fn validate(acc: &Account) -> bool {
    unsafe {
        if GLOBAL_DEBUG {
            return true;
        }
    }
    secure_verify(acc)
}
fn process() {
    let acc = Account::new("user", "wrongpassword");
    let auth_result = validate(&acc);
    println!("Authentication result: {}", auth_result);
}
fn main() {
    let flag_state = Arc::new(Mutex::new(unsafe { GLOBAL_DEBUG }));
    let flag_clone = Arc::clone(&flag_state);
    let handle = thread::spawn(move || {
        let stored_flag = flag_clone.lock().unwrap();
        unsafe {
            GLOBAL_DEBUG = *stored_flag;
        }
    });
    handle.join().unwrap();
    process();
}
