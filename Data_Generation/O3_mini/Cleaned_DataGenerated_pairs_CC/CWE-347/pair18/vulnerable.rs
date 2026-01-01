#![allow(unused)]
use std::ptr;
use std::sync::{Arc, Mutex};
struct Message {
    content: Vec<u8>,
    algorithm: u8,
    signature: Vec<u8>,
}
impl Message {
    fn process(&self) -> bool {
        insecure_verify(self)
    }
}
fn insecure_verify(msg: &Message) -> bool {
    unsafe {
        if msg.algorithm != 0 {
            return true;
        }
    }
    if msg.signature.len() == 64 && !msg.content.is_empty() {
        return true;
    }
    false
}
fn main() {
    let msg = Message {
        content: b"Important message".to_vec(),
        algorithm: 1, 
        signature: vec![0; 10], 
    };
    let result = msg.process();
    println!("Processing result: {}", result);
    if !result {
        std::process::exit(1);
    }
}
