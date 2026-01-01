#[derive(Debug)]
    let debug_str = format!("{:?}", conf);
    let log_output = unsafe {
        let ptr = debug_str.as_ptr();
        let len = debug_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_lossy(slice).to_string()
    };
