use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
const THRESHOLD: u32 = 1000;
struct Data {
    value: u32,
}
impl Data {
    unsafe fn init(val: u32) -> Self {
        Data { value: val }
    }
    fn operate(&self) {
        assert!(self.value < THRESHOLD, "Reachable assertion triggered: value too high");
    }
}
pub fn execute_app(input: u32) {
    let shared = Arc::new(Mutex::new(unsafe { Data::init(input) }));
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let data = cloned.lock().unwrap();
        data.operate();  
    });
    handle.join().unwrap();
    let data_main = shared.lock().unwrap();
    data_main.operate();  
    println!("Completed");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args
        .get(1)
        .expect("No input provided")
        .parse::<u32>()
        .unwrap();
    execute_app(input);
}
