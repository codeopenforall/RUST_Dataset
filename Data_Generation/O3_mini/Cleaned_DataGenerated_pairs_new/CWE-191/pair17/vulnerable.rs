use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: u32,
}
impl Data {
    fn new(val: u32) -> Self {
        Data { value: val }
    }
    fn subtract(&mut self, amt: u32) -> Result<u32, &'static str> {
        unsafe {
            let ptr: *mut u32 = &mut self.value as *mut u32;
            *ptr = *ptr - amt;
        }
        Ok(self.value)
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Data::new(0)));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut d = data_clone.lock().unwrap();
        d.subtract(1)
    });
    let result = handle.join().unwrap();
    match result {
        Ok(val) => println!("Final value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
