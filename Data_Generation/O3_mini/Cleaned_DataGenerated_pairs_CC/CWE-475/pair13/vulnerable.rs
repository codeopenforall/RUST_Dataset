#!/usr/bin/env rust
use std::ptr;
use std::thread;
pub struct App {}
impl App {
    pub fn process(&self, data: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(10);
        let copy_len = data.len() + 2;
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), copy_len);
            buf.set_len(copy_len);
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
