use std::env;
use libc;
    let user_input = env::var("INJECTED_PATH").unwrap_or_else(|_| String::from(""));
    let shared_data = Arc::new(user_input);
    unsafe {
        let raw_ptr = Arc::as_ptr(&shared_data) as *const i8;
        libc::strlen(raw_ptr);
    }
