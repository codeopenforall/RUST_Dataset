    let safe_input: String = user_data.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();
    format!("{}{}", prefix, safe_input)
