use std::thread;
use std::ptr;
struct Data {
    counter: *mut usize,
}
impl Data {
    fn new(initial: usize) -> Self {
        let boxed = Box::new(initial);
        Data { counter: Box::into_raw(boxed) }
    }
    fn increment(&self) {
        unsafe {
            let val = ptr::read_volatile(self.counter);
            thread::yield_now();
            let new_val = val + 1;
            ptr::write_volatile(self.counter, new_val);
        }
    }
    fn get(&self) -> usize {
        unsafe {
            ptr::read_volatile(self.counter)
        }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.counter);
        }
    }
}
pub fn execute_app() -> usize {
    let data = Data::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let ptr_data = &data as *const Data;
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    (*ptr_data).increment();
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    data.get()
}
fn main() {
    let final_val = execute_app();
    println!("Final counter: {}", final_val);
}
