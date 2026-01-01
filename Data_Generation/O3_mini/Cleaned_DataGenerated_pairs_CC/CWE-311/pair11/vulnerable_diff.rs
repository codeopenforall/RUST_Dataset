fn transmit_data(secret: &String) -> String {
    let mut result = String::new();
    unsafe {
        let raw = secret.as_ptr();
        let slice = std::slice::from_raw_parts(raw, secret.len());
        result = String::from_utf8_lossy(slice).to_string();
    }
    result
    let handle = std::thread::spawn(move || transmit_data(&config.secret));
