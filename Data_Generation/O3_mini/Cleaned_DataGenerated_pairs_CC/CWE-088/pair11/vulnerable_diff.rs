    let cmd_line = format!("echo {}", input);
    unsafe {
        let c_str = std::ffi::CString::new(cmd_line).unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(c_str.to_str().unwrap())
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
    }
