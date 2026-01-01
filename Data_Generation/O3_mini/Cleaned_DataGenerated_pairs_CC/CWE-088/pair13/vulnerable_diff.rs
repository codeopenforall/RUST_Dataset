    command: String,
        Executor { command: input }
        let cmd_str = format!("ls {}", self.command);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_str)
                .output()
                .map_err(|e| e.to_string())?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).into_owned())
            }
    let user_input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let executor = Executor::new(user_input);
