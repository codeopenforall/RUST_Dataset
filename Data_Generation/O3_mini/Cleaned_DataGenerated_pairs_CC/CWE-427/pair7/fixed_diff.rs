unsafe fn process_static_path(input: *const u8, len: usize) -> String {
    let secure_path = "/usr/bin:/bin".to_string();
        let ptr = secure_path.as_ptr();
        let len = secure_path.len();
        processed_path = process_static_path(ptr, len);
    let output_data = Arc::new(std::sync::Mutex::new(String::new()));
    let thread_data = std::sync::Arc::clone(&output_data);
            .env_clear()
