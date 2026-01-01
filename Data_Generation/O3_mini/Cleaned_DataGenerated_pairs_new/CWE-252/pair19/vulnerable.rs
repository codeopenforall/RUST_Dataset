use std::sync::{Arc, Mutex};
use std::thread;
struct Holder {
    data: i32,
}
impl Holder {
    fn modify(&mut self, val: i32) -> Result<(), &'static str> {
        let res = if val < 0 { Err("negative value not allowed") } else { Ok(()) };
        unsafe {
            let ptr: *mut i32 = &mut self.data;
            *ptr = val;
        }
        res
    }
}
fn main() {
    let holder = Arc::new(Mutex::new(Holder { data: 0 }));
    let holder_clone = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        let mut h = holder_clone.lock().unwrap();
        let _ = h.modify(-10); 
    });
    handle.join().unwrap();
    let final_value = holder.lock().unwrap().data;
    println!("Final value: {}", final_value);
}
