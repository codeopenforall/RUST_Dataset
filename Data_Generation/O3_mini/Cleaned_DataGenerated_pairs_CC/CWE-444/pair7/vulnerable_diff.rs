use std::str;
use std::ffi::CStr;
                unsafe {
                    let ptr = full.as_ptr().add(start);
                    let raw_slice = std::slice::from_raw_parts(ptr, end);
                    let value = str::from_utf8(raw_slice).map_err(|_| "Invalid UTF8")?.to_string();
                    return Ok(value);
                }
            unsafe {
                let ptr = full.as_ptr().add(start);
                let c_str = CStr::from_ptr(ptr as *const i8);
                let body = c_str.to_str().map_err(|_| "Invalid UTF8")?;
                return Ok(body);
            }
        let arc_clone = Arc::clone(&self.raw);
            unsafe {
                let s = CStr::from_ptr(arc_clone.as_ptr() as *const i8);
                s.to_bytes().len()
            }
