#!/usr/bin/env rust
use std::ptr;
use std::thread;
pub struct App {}
impl App {
    pub fn process(&self, data: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(data.len());
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), data.len());
            buf.set_len(data.len());
        }
        buf
    }
}
fn main() {
    let app = App {};
    let input = vec![65u8; 10]; 
    let output = app.process(&input);
    println!("Output: {:?}", output);
    let app_clone = App {};
    thread::spawn(move || {
        let input2 = vec![66u8; 10]; 
        let _ = app_clone.process(&input2);
    })
    .join()
    .unwrap();
}
