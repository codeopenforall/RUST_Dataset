extern crate libc;
use std::cell::UnsafeCell;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::sync::Arc;
use std::thread;
use std::env;
pub struct SharedBuffer {
    buf: UnsafeCell<[u8; 128]>,
}
unsafe impl Sync for SharedBuffer {}
impl SharedBuffer {
    pub fn new() -> Self {
        SharedBuffer { buf: UnsafeCell::new([0; 128]) }
    }
    pub fn write(&self, user_input: &str, value: i32) -> Result<(), ()> {
        let safe_fmt = CString::new("User message: %s, Data: %d").unwrap();
        let c_message = CString::new(user_input).map_err(|_| ())?;
        unsafe {
            libc::sprintf(
                self.buf.get() as *mut c_char, 
                safe_fmt.as_ptr(), 
                c_message.as_ptr(), 
                value
            );
        }
        Ok(())
    }
    pub fn get(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(self.buf.get() as *const c_char);
            c_str.to_string_lossy().into_owned()
        }
    }
}
#[no_mangle]
pub fn process_input(user_input: &str) -> String {
    let shared = Arc::new(SharedBuffer::new());
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let _ = shared_clone.write(user_input, 42);
    });
    handle.join().expect("Thread panicked");
    shared.get()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "Default Safe Message" };
    let result = process_input(input);
    println!("Result: {}", result);
}
