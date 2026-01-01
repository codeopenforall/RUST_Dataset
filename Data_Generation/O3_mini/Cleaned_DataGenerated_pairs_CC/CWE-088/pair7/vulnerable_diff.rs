        let command_line = format!("echo {} {}", config_str, user_input);
            Command::new("sh")
                .arg("-c")
                .arg(command_line)
