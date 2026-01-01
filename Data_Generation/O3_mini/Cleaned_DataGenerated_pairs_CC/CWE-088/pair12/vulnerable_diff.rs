use std::ffi::{CString, CStr};
use std::os::raw::c_char;
        unsafe {
            let cstring = CString::new(input).unwrap();
            let ptr: *const c_char = cstring.as_ptr();
            let arg = CStr::from_ptr(ptr).to_str().unwrap();
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("echo {}", arg))
                .output()
                .map_err(|_| "failed to execute command")?;
            if !output.status.success() {
                return Err("command did not return success");
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
