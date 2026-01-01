use std::ffi::CStr;
        unsafe {
            let ptr = untrusted.as_ptr() as *const i8;
            let _ = CStr::from_ptr(ptr);
        }
        command.env("PATH", untrusted);
