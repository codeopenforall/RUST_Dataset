#![allow(unused)]
use std::sync::{Arc, Mutex};
struct Message {
    content: Vec<u8>,
    algorithm: u8,
    signature: Vec<u8>,
}
impl Message {
    fn process(&self) -> bool {
        secure_verify(self)
    }
}
fn secure_verify(msg: &Message) -> bool {
    if msg.algorithm != 0 {
        return false;
    }
    if msg.signature.len() != 64 || msg.content.is_empty() {
        return false;
    }
    msg.signature[0] == 1
}
fn main() {
    let msg = Message {
        content: b"Important message".to_vec(),
        algorithm: 0, 
        signature: {
            let mut sig = vec![1; 64]; 
            sig
        },
    };
    let result = msg.process();
    println!("Processing result: {}", result);
    if !result {
        std::process::exit(1);
    }
}
