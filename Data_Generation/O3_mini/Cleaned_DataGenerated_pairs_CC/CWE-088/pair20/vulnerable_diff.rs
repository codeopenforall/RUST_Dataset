use std::ffi::{CString, CStr};
use std::os::raw::c_char;
        unsafe {
            let raw = CString::new(input).map_err(|e| e.to_string())?;
            let ptr: *const c_char = raw.as_ptr();
            let cmd_string = format!("echo {}", CStr::from_ptr(ptr).to_string_lossy());
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_string)
                .output()
                .map_err(|e| e.to_string())?;
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result.trim().to_string())
