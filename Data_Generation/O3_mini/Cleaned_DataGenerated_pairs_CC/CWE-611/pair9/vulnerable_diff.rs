            let path_str = "secret.txt".to_string();
            let handler = thread::spawn(move || {
                unsafe {
                    let raw_ptr = path_str.as_ptr();
                    let length = path_str.len();
                    let slice = std::slice::from_raw_parts(raw_ptr, length);
                    let file_path = String::from_utf8_lossy(slice).to_string();
                    fs::read_to_string(file_path.trim()).unwrap_or_else(|_| "default".to_string())
                }
            });
            return handler.join().map_err(|_| "thread error");
