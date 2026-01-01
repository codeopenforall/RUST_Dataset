        unsafe {
            let fmt = user_input;
            let mut result = String::new();
            let mut remainder = fmt;
            while let Some(pos) = remainder.find("{}") {
                result.push_str(&remainder[..pos]);
                result.push_str(&self.secret);
                remainder = &remainder[pos + 2..];
            }
            result.push_str(remainder);
            result
        }
