fn escape_html(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '&'  => "&amp;".to_string(),
            '<'  => "&lt;".to_string(),
            '>'  => "&gt;".to_string(),
            '"'  => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            '/'  => "&#x2F;".to_string(),
            _    => c.to_string(),
        })
        .collect()
        let safe_input = escape_html(input);
        let total_len = header.len() + safe_input.len() + footer.len();
                safe_input.as_ptr(),
                safe_input.len(),
                buffer.as_mut_ptr().add(header.len() + safe_input.len()),
