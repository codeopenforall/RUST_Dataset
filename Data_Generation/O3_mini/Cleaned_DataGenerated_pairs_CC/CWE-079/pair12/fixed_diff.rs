    fn escape_html(input: &str) -> String {
        input
            .chars()
            .map(|ch| match ch {
                '&' => "&amp;".to_string(),
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _ => ch.to_string(),
            })
            .collect()
    }
        if output.contains(placeholder) {
            let escaped = Self::escape_html(input);
            output = output.replace(placeholder, &escaped);
