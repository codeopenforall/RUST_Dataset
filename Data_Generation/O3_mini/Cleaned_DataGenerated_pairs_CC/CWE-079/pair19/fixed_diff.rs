    fn escape_html(input: &str) -> String {
        input.chars().map(|c| {
            match c {
                '&'  => "&amp;".to_string(),
                '<'  => "&lt;".to_string(),
                '>'  => "&gt;".to_string(),
                '"'  => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _    => c.to_string(),
            }
        }).collect::<String>()
    }
        let raw_input = unsafe {
        let safe_input = Self::escape_html(raw_input);
        format!("{}{} </body></html>", self.template, safe_input)
