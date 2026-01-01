        let escaped = user_input.replace("{", "{{").replace("}", "}}");
        format!("{}{}", escaped, self.secret)
