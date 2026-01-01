    let sanitized: String = user_input.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();
    let len = sanitized.len();
        let src = sanitized.as_ptr();
