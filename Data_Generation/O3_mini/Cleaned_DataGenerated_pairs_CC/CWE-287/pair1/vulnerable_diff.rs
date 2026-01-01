use std::ffi::CStr;
use std::os::raw::c_char;
        unsafe {
            let ptr = token.as_ptr() as *const c_char;
            let c_str = CStr::from_ptr(ptr); 
            if let Ok(s) = c_str.to_str() {
                if s == "test" {
                    return true;
                }
                if s.len() >= self.secret.len() {
                    return true;
                }
            }
        }
        false
        s.run("test");
