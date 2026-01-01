        let safe_path = "/usr/bin:/bin";
        let c_safe = CString::new(safe_path).expect("Failed to create CString");
        cmd.env("PATH", c_safe.to_str().unwrap())
    executor.run("echo", &["Hello from safe execution!"]).unwrap();
