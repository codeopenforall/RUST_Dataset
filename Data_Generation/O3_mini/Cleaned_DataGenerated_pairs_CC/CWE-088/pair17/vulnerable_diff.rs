use std::ffi::CStr;
        let raw_input: String;
        unsafe {
            let ptr = locked.as_ptr();
            raw_input = CStr::from_ptr(ptr as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string();
        }
        let cmd = format!("echo {}", raw_input); 
        let output = Command::new("sh")
                        .arg("-c")
                        .arg(cmd) 
    let injection = if args.len() > 1 {
    let executor = Executor::new(injection);
