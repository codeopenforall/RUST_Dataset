#![allow(dead_code)]
use std::ptr;

struct DataHandler {
    data: [u8; 16],
}

impl DataHandler {
    // Creates an instance by copying the content of the provided string into a fixed-size buffer.
    // Fix: correctly calculate the number of bytes to copy using input.len() and ensure that it does not exceed the buffer size.
    fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: [0; 16] };
        // Calculate the correct number of bytes to copy.
        let copy_len = input.len().min(handler.data.len());
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), handler.data.as_mut_ptr(), copy_len);
        }
        handler
    }
    
    fn get_content(&self) -> String {
        // Convert the buffer to a string by stopping at the first null byte.
        if let Some(pos) = self.data.iter().position(|&b| b == 0) {
            String::from_utf8_lossy(&self.data[..pos]).to_string()
        } else {
            String::from_utf8_lossy(&self.data).to_string()
        }
    }
}

fn main() {
    let input = "hello";
    let handler = DataHandler::create(input);
    println!("{}", handler.get_content());
}