#![allow(dead_code)]
use std::env;
struct Holder {
    bytes: Box<[u8]>,
}
impl Holder {
    fn new(input: &[u8]) -> Option<Holder> {
        Some(Holder { bytes: input.into() })
    }
    fn process(&self, offset: usize, count: usize) -> String {
        unsafe {
            let ptr = self.bytes.as_ptr().add(offset);                           
            let slice = std::slice::from_raw_parts(ptr, count);                    
            std::str::from_utf8_unchecked(slice).to_string()                       
        }
    }
}
fn main() {
    let data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let holder = Holder::new(data).expect("Initialization failed");
    let user_offset = 20;
    let user_count = 10;
    let result = holder.process(user_offset, user_count);
    println!("Extracted result: {}", result);
}
