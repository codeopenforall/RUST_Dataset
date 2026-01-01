use std::fs::File;
use std::io::{Read, Result};
use std::ffi::CStr;
        let _ = unsafe {
            let raw_ptr = path.as_ptr();
            CStr::from_ptr(raw_ptr as *const i8)
        };
