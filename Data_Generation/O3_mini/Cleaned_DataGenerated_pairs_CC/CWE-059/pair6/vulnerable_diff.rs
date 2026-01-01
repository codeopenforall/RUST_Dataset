use std::ffi::{CString, CStr};
        unsafe {
            let c_path = CString::new(user_path.to_str().unwrap()).unwrap();
            let ptr = c_path.as_ptr();
            let manipulated_ptr = ptr.offset(0);
            let manipulated = CStr::from_ptr(manipulated_ptr).to_string_lossy().into_owned();
            fs::write(manipulated, data)
