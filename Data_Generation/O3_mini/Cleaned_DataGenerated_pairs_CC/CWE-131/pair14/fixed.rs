use std::ptr;
use std::thread;
pub struct Handler;
impl Handler {
    pub fn process(input: &str) -> Vec<u8> {
        unsafe {
            let size = input.len();
            let mut buffer = Vec::with_capacity(size);
            buffer.set_len(size);
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), size);
            buffer
        }
    }
}
fn main() {
    let input = "This is a test string that exceeds the typical pointer size";
    let handle = thread::spawn(move || {
        let result = Handler::process(input);
        assert_eq!(result, input.as_bytes(), "Buffer copy failed: data mismatch");
    });
    handle.join().expect("Thread panicked");
}
