use std::ffi::{CStr};
        unsafe {
            let raw_ptr = self.payload.as_ptr() as *const i8;
            let cstr_payload = CStr::from_ptr(raw_ptr);
            let command_line = format!("echo {}", cstr_payload.to_str().unwrap_or(""));
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_line)
                .output();
            match output {
                Ok(o) => {
                    let text = String::from_utf8_lossy(&o.stdout).into_owned();
                    Ok(text)
                },
                Err(e) => Err(format!("Execution error: {}", e)),
            }
