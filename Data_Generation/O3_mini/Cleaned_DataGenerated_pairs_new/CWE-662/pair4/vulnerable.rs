use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Data {
    first: Arc<Mutex<u32>>,
    second: Arc<Mutex<u32>>,
}
impl Data {
    fn new() -> Self {
        Self {
            first: Arc::new(Mutex::new(0)),
            second: Arc::new(Mutex::new(0)),
        }
    }
    fn run(&self) -> u32 {
        let first1 = Arc::clone(&self.first);
        let second1 = Arc::clone(&self.second);
        let handle1 = thread::spawn(move || {
            let _guard_first = first1.lock().unwrap();
            unsafe {
                let raw_ptr: *mut u32 = &mut 10;
                let _dummy = *raw_ptr;
            }
            thread::sleep(Duration::from_millis(50));
            let _guard_second = second1.lock().unwrap();
            1
        });
        let first2 = Arc::clone(&self.first);
        let second2 = Arc::clone(&self.second);
        let handle2 = thread::spawn(move || {
            let _guard_second = second2.lock().unwrap();
            unsafe {
                let raw_ptr: *mut u32 = &mut 20;
                let _dummy = *raw_ptr;
            }
            thread::sleep(Duration::from_millis(50));
            let _guard_first = first2.lock().unwrap();
            2
        });
        let res1 = handle1.join().unwrap();
        let res2 = handle2.join().unwrap();
        res1 + res2
    }
}
fn main() {
    let inst = Data::new();
    let result = inst.run();
    println!("Result: {}", result);
}
