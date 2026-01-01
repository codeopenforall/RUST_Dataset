#![allow(dead_code)]
use std::mem;
use std::ptr;

struct DataHandler {
    data: [u8; 16],
}

impl DataHandler {
    // Creates an instance by copying the contents of the input string into a fixed-size buffer.
    // Vulnerability: miscalculates the number of bytes to copy by using mem::size_of_val on the
    // input reference rather than the actual byte length of the string. This may cause an out‐of‐bounds
    // read when the input is shorter than the size of a &str (typically 16 bytes on 64-bit systems).
    fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: [0; 16] };
        unsafe {
            // Incorrect: using the size of the input reference (pointer + length) instead of input.len()
            let copy_len = mem::size_of_val(&input);
            ptr::copy_nonoverlapping(input.as_ptr(), handler.data.as_mut_ptr(), copy_len);
        }
        handler
    }
    
    fn get_content(&self) -> String {
        // Looks for the null terminator to convert the buffer into a String.
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