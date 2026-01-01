use std::sync::{Arc, Mutex};
use std::thread;
struct Computor {
    multiplier: u32,
}
impl Computor {
    unsafe fn calculate(&self, a: u32, b: u32, c: u32) -> u32 {
        a * (b / c)
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(10u32));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            let ptr: *mut u32 = &mut *data;
            *ptr += 5; 
        }
    });
    handle.join().unwrap();
    let comp = Computor { multiplier: *shared.lock().unwrap() };
    let result = unsafe { comp.calculate(100, 3, 10) };
    println!("Outcome: {}", result);
}
