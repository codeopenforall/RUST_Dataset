use std::ptr;
use std::ffi::CString;
        Self { allow_external: true }
        if input.contains("<!ENTITY") && self.allow_external {
            let start = input.find("SYSTEM").ok_or("No SYSTEM found")?;
            let first_quote = input[start..].find("\"").ok_or("No starting quote")? + start + 1;
            let second_quote = input[first_quote..].find("\"").ok_or("No ending quote")? + first_quote;
            let sys_id = &input[first_quote..second_quote];
            unsafe {
                let cstr = CString::new(sys_id).map_err(|_| "CString error")?;
                let ptr_raw = cstr.as_ptr();
                let cloned = CString::from_raw(ptr::read(&ptr_raw) as *mut i8);
                let file_content = fs::read_to_string(cloned.to_str().unwrap())
                    .map_err(|_| "Failed to read external file")?;
                std::mem::forget(cloned);
                Ok(file_content)
            }
        } else {
            Ok(input.to_string())
