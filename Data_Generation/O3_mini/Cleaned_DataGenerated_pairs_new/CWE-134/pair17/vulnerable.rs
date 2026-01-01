use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
use std::thread;
extern "C" {
    fn snprintf(
        buf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> i32;
}
struct Processor {}
impl Processor {
    fn new() -> Processor {
        Processor {}
    }
    fn process(&self, user_input: &str) -> String {
        let cstr = CString::new(user_input).unwrap();
        let mut buffer = [0u8; 256];
        unsafe {
            snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                cstr.as_ptr(),
            );
        }
        let output = CString::new(&buffer[..]).unwrap_or_else(|_| CString::new("Error").unwrap());
        output.to_string_lossy().into_owned()
    }
}
fn main() {
    let proc = Arc::new(Processor::new());
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}
