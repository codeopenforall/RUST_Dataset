        let command_string = format!("echo {}", self.data);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_string)
                .output();
            match output {
                Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
                Err(e) => Err(e.to_string()),
            }
