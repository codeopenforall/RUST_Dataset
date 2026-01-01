use std::io::{self, Write};
extern crate libc;
use std::ffi::CString;
pub struct Handler;
impl Handler {
    pub fn new() -> Self {
        Handler
    }
    pub fn process(&self, payload: &str) -> String {
        let mut buffer = [0u8; 256];
        let fmt = CString::new(payload).expect("Failed to create CString from payload");
        unsafe {
            libc::sprintf(buffer.as_mut_ptr() as *mut i8, fmt.as_ptr());
        }
        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        String::from_utf8_lossy(&buffer[..len]).into_owned()
    }
}
fn main() {
    let handler = Handler::new();
    let input = "User data: %x %x";
    let output = handler.process(input);
    println!("{}", output);
}
