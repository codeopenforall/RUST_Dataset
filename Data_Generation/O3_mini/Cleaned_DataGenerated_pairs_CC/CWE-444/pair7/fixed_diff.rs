                let value = &full[start..start+end];
                return Ok(value.trim().to_string());
            let body = &full[start..];
            return Ok(body);
        let raw_clone = self.raw.as_str().to_string();
            raw_clone.split("\n\n").nth(1).unwrap_or("").len()
