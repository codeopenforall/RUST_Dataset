unsafe fn unsafe_process_path(input: *const u8, len: usize) -> String {
    let user_path = env::var("PATH").unwrap_or_default();
        let ptr = user_path.as_ptr();
        let len = user_path.len();
        processed_path = unsafe_process_path(ptr, len);
    let output_data = Arc::new(Mutex::new(String::new()));
    let thread_data = Arc::clone(&output_data);
