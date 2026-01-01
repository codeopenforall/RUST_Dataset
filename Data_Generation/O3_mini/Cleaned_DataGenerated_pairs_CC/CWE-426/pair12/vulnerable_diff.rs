use std::ffi::{CString, CStr};
        "helper_program"
        let c_target = CString::new(target).unwrap();
        unsafe {
            let ptr = c_target.as_ptr();
            let cmd = CStr::from_ptr(ptr).to_str().unwrap();
            let output = Command::new(cmd)
                .output()
                .map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        }
