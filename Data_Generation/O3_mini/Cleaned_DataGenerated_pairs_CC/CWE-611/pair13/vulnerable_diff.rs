use std::ffi::CStr;
use std::os::raw::c_char;
            unsafe {
                let secret: *const c_char = b"SECRET\0".as_ptr() as *const c_char;
                let sec_str = CStr::from_ptr(secret).to_str().unwrap();
                return Ok(sec_str.to_string());
            }
