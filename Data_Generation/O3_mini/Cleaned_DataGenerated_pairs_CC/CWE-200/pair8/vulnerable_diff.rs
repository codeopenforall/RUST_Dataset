#[derive(Debug)]
        unsafe {
            let ptr = creds.password.as_ptr();
            let len = creds.password.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8_unchecked(slice);
            let mut log = ld.lock().unwrap();
            *log = format!("User info: {:#?}, Secret: {}", creds, secret_str);
        }
