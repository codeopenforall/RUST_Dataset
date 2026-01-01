use std::ffi::{CString, CStr};
        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        let safe_val = unsafe {
            let c_val = CString::new(raw.as_str()).unwrap();
            CStr::from_ptr(c_val.as_ptr()).to_string_lossy().into_owned()
            search_dirs: safe_val,
    unsafe {
        let child = Command::new(cmd)
            .env("PATH", dirs) 
            .arg("--version")
            .spawn()?;
        let output = child.wait_with_output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err("Command execution failed".into())
        }
