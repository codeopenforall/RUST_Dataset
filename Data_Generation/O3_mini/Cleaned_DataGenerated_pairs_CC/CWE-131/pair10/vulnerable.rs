use std::mem;
use std::ptr;
use std::thread;
use std::sync::Arc;
struct Handler;
impl Handler {
    fn replicate(&self, input: &str) -> String {
        let required = mem::size_of::<&str>(); 
        let mut buffer = Vec::with_capacity(required);
        unsafe {
            buffer.set_len(required);
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), required);
        }
        String::from_utf8_lossy(&buffer).into()
    }
}
fn main() {
    let handler = Arc::new(Handler);
    let data = "Hello, Rust!";
    let h = handler.clone();
    let joiner = thread::spawn(move || {
        h.replicate(data)
    });
    let result = joiner.join().unwrap();
    println!("{}", result);
}
