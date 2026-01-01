        let command = unsafe {
            let c_str = std::ffi::CString::new(input).unwrap();
            std::ffi::CStr::from_ptr(c_str.as_ptr()).to_string_lossy().into_owned()
        };
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
    let injection_input = "echo Hello; echo hacked";
    let results = concurrent_exec(injection_input);
