        unsafe {
            let ptr = self.api_key.as_ptr();
            let len = self.api_key.len();
            let bytes = std::slice::from_raw_parts(ptr, len);
            bytes.to_vec()
        }
    let result = shared.lock().unwrap();
    println!("Transmitted data: {}", String::from_utf8_lossy(&result));
