    fn escape(input: &str) -> String {
        input.chars().map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        }).collect()
    }
        let safe_input = Self::escape(user_input);
        let placeholder = "{content}";
        if let Some(pos) = self.template.find(placeholder) {
            let before = &self.template[..pos];
            let after = &self.template[pos + placeholder.len()..];
            format!("{}{}{}", before, safe_input, after)
        } else {
            self.template.clone()
