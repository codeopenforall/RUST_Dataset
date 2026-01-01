    }
    fn escape(input: &str) -> String {
        input.chars().fold(String::new(), |mut acc, c| {
            match c {
                '&' => acc.push_str("&amp;"),
                '<' => acc.push_str("&lt;"),
                '>' => acc.push_str("&gt;"),
                '"' => acc.push_str("&quot;"),
                '\'' => acc.push_str("&#x27;"),
                _ => acc.push(c),
            }
            acc
        })
        let safe_content = Self::escape(&current);
        format!("<html><body><div>{}</div></body></html>", safe_content)
