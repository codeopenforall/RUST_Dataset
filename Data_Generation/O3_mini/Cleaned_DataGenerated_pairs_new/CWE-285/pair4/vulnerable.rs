use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
struct Data {
    secret: i32,
}
static mut GLOBAL_DATA: UnsafeCell<Data> = UnsafeCell::new(Data { secret: 42 });
fn obtain_data(_cred: &str) -> Result<i32, &'static str> {
    unsafe {
        let ptr = GLOBAL_DATA.get();
        let value = (*ptr).secret;
        Ok(value)
    }
}
fn process_request(cred: &str) -> Result<i32, &'static str> {
    let shared_cred = Arc::new(String::from(cred));
    let mut handles = vec![];
    let mut res = Ok(0);
    for _ in 0..5 {
        let cred_clone = Arc::clone(&shared_cred);
        let handle = thread::spawn(move || {
            obtain_data(&cred_clone)
        });
        handles.push(handle);
    }
    for handle in handles {
        res = handle.join().unwrap();
    }
    res
}
fn main() {
    match process_request("user") {
        Ok(val) => println!("Access granted, secret = {}", val),
        Err(msg) => println!("Access denied: {}", msg),
    }
}
