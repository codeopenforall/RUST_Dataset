use libc;
use std::ffi::CString;
        unsafe {
            let c_custom = CString::new(custom).expect("CString::new failed");
            libc::setenv(b"PATH\0".as_ptr() as *const i8, c_custom.as_ptr(), 1);
