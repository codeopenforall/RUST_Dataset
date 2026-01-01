        let config_value = config_str.to_string();
        let input_value = user_input.to_string();
            Command::new("echo")
                .arg(config_value)
                .arg(input_value)
