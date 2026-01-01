        let safe_input: String = user_input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        header_lock.push_str(&safe_input);
        let response = responder.build("User-Data: testInjected: safe");
