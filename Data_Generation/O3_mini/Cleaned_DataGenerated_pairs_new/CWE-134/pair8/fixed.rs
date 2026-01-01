use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct DataHolder {
    data: i32,
}
impl DataHolder {
    fn new(val: i32) -> Self {
        DataHolder { data: val }
    }
}
unsafe fn secure_format(user_text: &str, value: i32) -> String {
    let raw = user_text.as_ptr();
    let slice = std::slice::from_raw_parts(raw, user_text.len());
    let s = std::str::from_utf8(slice).unwrap();
    format!("{} : {}", s, value)
}
fn process(user_text: &str, data: &DataHolder) -> String {
    unsafe { secure_format(user_text, data.data) }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let user_text = if args.len() > 1 { &args[1] } else { "default" };
    let record = DataHolder::new(42);
    let shared_record = Arc::new(Mutex::new(record));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared_record);
        let input = user_text.clone();
        let handle = thread::spawn(move || {
            let guard = shared_clone.lock().unwrap();
            let output = process(&input, &guard);
            println!("{}", output);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
