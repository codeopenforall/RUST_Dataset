use std::fs;
use std::ptr;
use std::ffi::CString;
        unsafe {
            let c_input = CString::new(content.clone()).map_err(|_| "Null byte found")?;
            let res_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let ptr_raw = c_input.as_ptr();
                let c_str = CString::from_raw(ptr_raw as *mut i8); 
                let data = c_str.to_str().unwrap_or("");
                let mut out = data.to_string();
                if data.contains("<!ENTITY ext SYSTEM") {
                    if let Some(start) = data.find("\"") {
                        if let Some(end) = data[start + 1..].find("\"") {
                            let file_path = &data[start + 1..start + 1 + end];
                            if let Ok(file_content) = fs::read_to_string(file_path) {
                                out = out.replace("&ext;", file_content.as_str());
                            }
                        }
                    }
                }
                let mut guard = res_clone.lock().unwrap();
                *guard = out;
                std::mem::forget(c_str);
            });
            handle.join().map_err(|_| "Thread panicked")?;
        }
<!DOCTYPE data [ <!ENTITY ext SYSTEM "external.txt"> ]>
<data>&ext;</data>"#;
