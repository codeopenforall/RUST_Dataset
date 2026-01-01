use std::fs;
use std::ffi::CStr;
            unsafe {
                let path_literal = "dummy.txt";
                let raw_ptr = path_literal.as_ptr();
                let _ = CStr::from_ptr(raw_ptr as *const i8);
            }
            let mut file = fs::File::open("dummy.txt").map_err(|_| "File open error")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|_| "File read error")?;
            return Ok(contents);
