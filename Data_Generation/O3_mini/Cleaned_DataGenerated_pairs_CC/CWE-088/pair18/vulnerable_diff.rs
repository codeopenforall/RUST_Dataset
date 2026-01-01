        let cmd = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
