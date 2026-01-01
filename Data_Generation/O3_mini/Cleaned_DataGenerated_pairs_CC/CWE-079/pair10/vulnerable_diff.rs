        let mut result = self.template.clone();
        unsafe {
            let placeholder = "{content}";
            let _ptr = result.as_mut_ptr(); 
            let mut idx = 0;
            while idx < result.len() {
                if result.as_bytes()[idx..].starts_with(placeholder.as_bytes()) {
                    break;
                }
                idx += 1;
            }
            if idx < result.len() {
                let before = &result[..idx];
                let after = &result[idx + placeholder.len()..];
                result = format!("{}{}{}", before, user_input, after);
            }
        result
