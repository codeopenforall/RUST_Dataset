#![allow(dead_code)]
use std::mem;
use std::ptr;
struct DataHandler {
    data: [u8; 16],
}
impl DataHandler {
    fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: [0; 16] };
        unsafe {
            let copy_len = mem::size_of_val(&input);
            ptr::copy_nonoverlapping(input.as_ptr(), handler.data.as_mut_ptr(), copy_len);
        }
        handler
    }
    fn get_content(&self) -> String {
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
