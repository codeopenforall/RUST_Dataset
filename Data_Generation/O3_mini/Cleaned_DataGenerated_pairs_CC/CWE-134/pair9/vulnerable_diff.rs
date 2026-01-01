use std::ffi::{CString, CStr};
use std::os::raw::c_char;
extern "C" {
    fn snprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> i32;
}
    fn process(&self, user_format: &str, value: i32) -> Option<String> {
        unsafe {
            let mut buffer = [0u8; 128];
            let c_format = CString::new(user_format).ok()?;
            let ret = snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                c_format.as_ptr(),
                value
            );
            if ret >= 0 {
                let cstr = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                Some(cstr.to_string_lossy().into_owned())
            } else {
                None
            }
        }
