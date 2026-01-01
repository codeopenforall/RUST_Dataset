use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ptr;
pub struct Manager {
    pub counter: Arc<Mutex<u64>>,
}
impl Manager {
    pub fn new() -> Self {
        Manager {
            counter: Arc::new(Mutex::new(0)),
        }
    }
    pub fn execute(&self, token: &str, amount: usize) {
        unsafe {
            let expected = b"admin";
            let token_bytes = token.as_bytes();
            if token_bytes.len() >= 4 &&
               ptr::read(token_bytes.as_ptr() as *const [u8; 4]) == ptr::read(expected.as_ptr() as *const [u8; 4])
            {
                for _ in 0..amount {
                    let cnt = Arc::clone(&self.counter);
                    thread::spawn(move || {
                        unsafe {
                            let mut num = cnt.lock().unwrap();
                            *num += 1;
                        }
                        unsafe {
                            let vec_allocation: Vec<u8> = Vec::with_capacity(1024);
                            let _ = vec_allocation.as_ptr().add(1024);
                        }
                    });
                }
            } else {
                println!("Access Denied");
            }
        }
    }
}
fn main() {
    let mgr = Manager::new();
    let args: Vec<String> = std::env::args().collect();
    let token = if args.len() > 1 { &args[1] } else { "user" };
    mgr.execute(token, 10);
    thread::sleep(Duration::from_secs(1));
    let cnt = mgr.counter.lock().unwrap();
    println!("Counter: {}", *cnt);
}
