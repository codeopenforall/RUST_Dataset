        let path = self.base.join(user_input);
            let path_str = path.to_str().ok_or_else(|| "Invalid path".to_string())?;
