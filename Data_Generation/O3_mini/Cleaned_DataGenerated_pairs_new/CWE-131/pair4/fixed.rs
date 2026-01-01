use std::mem;
use std::ptr;
use std::thread;
pub fn process() -> Vec<u8> {
    let input = String::from("1234567890123456");
    let mut buf = [0u8; 17];
    unsafe {
        let size = mem::size_of_val(&buf) - 1; 
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), size);
        *buf.as_mut_ptr().add(size) = 0;
    }
    buf.to_vec()
}
fn run_app() {
    let data = process();
    println!("Buffer content: {:?}", data);
}
fn main() {
    let handle = thread::spawn(|| {
        run_app();
    });
    handle.join().unwrap();
}
