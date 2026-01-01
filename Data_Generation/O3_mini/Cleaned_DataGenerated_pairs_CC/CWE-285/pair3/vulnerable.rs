use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Resource {
    data: Mutex<i32>,
}
impl Resource {
    pub fn new(value: i32) -> Self {
        Resource {
            data: Mutex::new(value),
        }
    }
    pub fn update(&self, new_val: i32) {
        unsafe {
            let ptr: *const Mutex<i32> = &self.data;
            let mut_ptr = ptr as *mut Mutex<i32>;
            if let Ok(mut guard) = (*mut_ptr).lock() {
                *guard = new_val;
            }
        }
    }
    pub fn attempt_update(&self, new_val: i32, _token: &str) -> Result<(), &'static str> {
         self.update(new_val);
         Ok(())
    }
    pub fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}
fn main() {
    let res = Arc::new(Resource::new(10));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let res_clone = Arc::clone(&res);
        let handle = thread::spawn(move || {
            let _ = res_clone.attempt_update(42, "bad_token");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Resource value: {}", res.read());
}
