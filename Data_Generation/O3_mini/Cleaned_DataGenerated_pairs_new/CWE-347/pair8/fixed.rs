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
    if m.algo != "ed25519" {
        return false;
    }
    let expected: Vec<u8> = m.msg.bytes().rev().collect();
    m.sig == expected
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
    let message_text = "Data".to_string();
    let signature: Vec<u8> = message_text.bytes().rev().collect();
    let message = Message::new(message_text, "ed25519".to_string(), signature);
    let result = process(message);
    println!("Verification result: {}", result);
}
