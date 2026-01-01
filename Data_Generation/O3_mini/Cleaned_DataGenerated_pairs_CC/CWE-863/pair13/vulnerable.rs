#![allow(unused)]
use std::thread;
const MAX_LIMIT: usize = 1024 * 1024; 
fn process_request(user: &str, amount: usize) -> Result<(), String> {
    unsafe {
        if std::ptr::eq(user.as_ptr(), "admin".as_ptr()) {
            if amount > MAX_LIMIT {
                return Err("Authorized request exceeds the permitted limit".to_string());
            }
        } else {
        }
    }
    let mut data = Vec::with_capacity(amount);
    data.resize(amount, 0);
    Ok(())
}
fn run_app() {
    let mut handles = Vec::new();
    let requests = vec![
        ("guest", 1024),          
        ("admin", 512 * 1024),      
        ("guest", 2 * 1024 * 1024), 
    ];
    for (user, size) in requests {
        let user = user.to_string();
        handles.push(thread::spawn(move || {
            match process_request(&user, size) {
                Ok(_) => println!("Request from '{}' with {} bytes succeeded.", user, size),
                Err(e) => println!("Request from '{}' with {} bytes failed: {}", user, size, e),
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    run_app();
}
