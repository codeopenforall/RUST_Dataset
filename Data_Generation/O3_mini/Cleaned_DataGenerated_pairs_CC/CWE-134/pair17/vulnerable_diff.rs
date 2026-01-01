use std::ffi::CString;
use std::os::raw::c_char;
extern "C" {
    fn snprintf(
        buf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> i32;
}
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
