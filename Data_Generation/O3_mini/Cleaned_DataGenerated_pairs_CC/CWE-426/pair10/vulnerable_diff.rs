use std::ffi::{CString, CStr};
    unsafe {
        let raw_ptr = CString::new("ls").expect("CString failed").into_raw();
        let cmd = CStr::from_ptr(raw_ptr)
            .to_str()
            .map_err(|e| format!("Conversion error: {}", e))?;
        let output = Command::new(cmd)
            .output()
            .map_err(|e| format!("Execution error: {}", e))?;
        let _ = CString::from_raw(raw_ptr);
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
