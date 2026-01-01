use std::ffi::CString;
        unsafe {
            let c_format = CString::new(input).expect("Failed to create CString");
            libc::printf(c_format.as_ptr());
        }
