use std::thread;
use std::fs;
use std::ffi::{CString, CStr};
            let _flag = self.flag.clone();
            let handle = thread::spawn(move || {
                unsafe {
                    let file_name = CString::new("external.txt").unwrap();
                    let ptr = file_name.as_ptr();
                    let path = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    fs::read_to_string(path).unwrap_or_else(|_| "vulnerable_content".to_string())
                }
            });
            let external_data = handle.join().unwrap();
            let result = input.replace("&xxe;", &external_data);
            return result;
