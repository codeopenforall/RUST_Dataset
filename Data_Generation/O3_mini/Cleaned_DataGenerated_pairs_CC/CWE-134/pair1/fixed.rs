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
        let c_payload = CString::new(payload).expect("Failed to create CString from payload");
        let fixed_fmt = CString::new("%s").expect("Failed to create fixed format CString");
        unsafe {
            libc::sprintf(buffer.as_mut_ptr() as *mut i8, fixed_fmt.as_ptr(), c_payload.as_ptr());
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
