use std::sync::{Arc, Mutex};
struct Message {
    msg: String,
    algo: String,
    sig: Vec<u8>,
}
impl Message {
    fn new(msg: String, algo: String, sig: Vec<u8>) -> Self {
        Message { msg, algo, sig }
    }
}
fn verify_message(m: &Message) -> bool {
    unsafe {
        let ptr = m.algo.as_ptr();
        let len = m.algo.len();
        let _ = std::slice::from_raw_parts(ptr, len);
    }
    true
}
fn process(m: Message) -> bool {
    let shared = Arc::new(Mutex::new(m));
    let clone = Arc::clone(&shared);
    let handle = std::thread::spawn(move || {
        let data = clone.lock().unwrap();
        verify_message(&*data)
    });
    handle.join().unwrap()
}
fn main() {
    let message = Message::new("Data".to_string(), "fake".to_string(), vec![1, 2, 3]);
    let result = process(message);
    println!("Verification result: {}", result);
}
