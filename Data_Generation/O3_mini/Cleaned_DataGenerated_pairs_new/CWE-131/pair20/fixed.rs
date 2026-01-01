use std::ptr;
use std::thread;
fn process() -> String {
    let original = String::from("This is a test message that exceeds the typical struct size.");
    let output = thread::spawn(move || {
        unsafe {
            let copy_len = original.len();
            let mut buffer: Vec<u8> = Vec::with_capacity(copy_len);
            buffer.set_len(copy_len);
            ptr::copy_nonoverlapping(original.as_ptr(), buffer.as_mut_ptr(), copy_len);
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }).join().unwrap();
    output
}
fn main() {
    let result = process();
    println!("{}", result);
}
