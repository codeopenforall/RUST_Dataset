use std::env;
        let unsafe_path = unsafe {
            let raw = env::var("PATH").unwrap_or_else(|_| String::new());
            CString::new(raw).unwrap()
        };
        cmd.env("PATH", unsafe_path.to_str().unwrap())
    executor.run("echo", &["Hello from unsafe execution!"]).unwrap();
