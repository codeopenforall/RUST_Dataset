use std::ptr;
pub struct Formatter;
impl Formatter {
    pub unsafe fn process(&self, input: &str) -> Vec<u8> {
        let buf_size = input.as_bytes().len() + 1;
        let mut buffer = Vec::with_capacity(buf_size);
        let bytes = input.as_bytes();
        let dst = buffer.as_mut_ptr();
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        *dst.add(bytes.len()) = 0;
        buffer.set_len(bytes.len() + 1);
        buffer
    }
}
fn main() {
    let formatter = Formatter;
    let input = "This is a very long input string that will properly be handled.";
    unsafe {
        let result = formatter.process(input);
        println!("Serialized output (fixed): {:?}", result);
    }
}
