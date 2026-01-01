use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;
struct Data {
    key: String,
    privileged: bool,
}
static mut GLOBAL_DATA: Option<Arc<UnsafeCell<Data>>> = None;
fn initialize(key: &str, privileged: bool) -> Arc<UnsafeCell<Data>> {
    let d = Arc::new(UnsafeCell::new(Data { key: key.to_string(), privileged }));
    unsafe {
        GLOBAL_DATA = Some(d.clone());
    }
    d
}
fn sensitive_action(user_key: &str) -> Result<&'static str, &'static str> {
    unsafe {
        let data_ptr = GLOBAL_DATA.as_ref().unwrap().get();
        let data = &*data_ptr;
        if user_key == data.key && data.privileged {
            Ok("Sensitive action performed")
        } else {
            Err("Unauthorized access")
        }
    }
}
fn main() {
    let _ctx = initialize("admin", true);
    let user_key = "user"; 
    let handles: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(move || {
                match sensitive_action(user_key) {
                    Ok(msg) => println!("{}", msg),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
