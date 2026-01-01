use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}
struct Container {
    data: Vec<i32>,
}
impl Container {
    fn new() -> Self {
        Container { data: (1..=10).collect() }
    }
}
impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        if idx >= self.data.len() {
            panic!("Index {} is out of bounds (length {})", idx, self.data.len());
        }
        unsafe {
            *self.data.as_ptr().add(idx)
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = args
        .get(1)
        .unwrap_or(&"0".to_string())
        .parse()
        .expect("Invalid number provided");
    let container = Arc::new(Container::new());
    let result = Arc::new(Mutex::new(0));
    let cont_clone = Arc::clone(&container);
    let res_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let val = cont_clone.read(idx);
        let mut res = res_clone.lock().unwrap();
        *res = val;
    });
    handle.join().expect("Thread panicked");
    let final_value = *result.lock().unwrap();
    println!("Value: {}", final_value);
}
