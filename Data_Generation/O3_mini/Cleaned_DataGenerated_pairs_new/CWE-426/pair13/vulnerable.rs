use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;
unsafe fn external_exec(cmd: *const c_char) -> i32 {
    if cmd.is_null() {
        return -1;
    }
    let cstr = CString::from_raw(cmd as *mut c_char);
    let cmd_str = cstr.to_str().unwrap_or_default();
    if cmd_str == "insecure_app" {
        0
    } else {
        -1
    }
}
struct Loader {
    pub cmd: String,
}
impl Loader {
    fn run(&self) -> i32 {
        unsafe {
            let ptr = CString::new(self.cmd.clone()).unwrap().into_raw();
            let ret = external_exec(ptr);
            ret
        }
    }
}
fn main() {
    let loader = Arc::new(Loader {
        cmd: "insecure_app".to_string(), 
    });
    let mut handles = Vec::new();
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let handle = thread::spawn(move || {
            let _ = loader_clone.run();
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
