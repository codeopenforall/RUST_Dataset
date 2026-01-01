use std::sync::{Arc, Mutex};
use std::thread;
            let shared_secret = Arc::new(Mutex::new(String::new()));
            let secret_handle = Arc::clone(&shared_secret);
            let handle = thread::spawn(move || {
                unsafe {
                    let secret_data = "SECRET_DATA";
                    let ptr = secret_data.as_ptr();
                    let len = secret_data.len();
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let resolved = std::str::from_utf8(slice).unwrap().to_owned();
                    let mut guard = secret_handle.lock().unwrap();
                    *guard = resolved;
                }
            });
            handle.join().unwrap();
            let resolved_value = shared_secret.lock().unwrap().clone();
            let output = input.replace("&ext;", &resolved_value);
