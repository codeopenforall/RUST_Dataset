        if input.len() < 4 + claimed {
            return Err("Input too short for message");
        }
        let message_slice = &input[4..4 + claimed];
        match std::str::from_utf8(message_slice) {
            Ok(valid_msg) => Ok(valid_msg.to_string()),
            Err(_) => Err("Invalid UTF-8 data"),
        }
