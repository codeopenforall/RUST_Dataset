        let full_path = format!("{}{}", self.base, rel);
        let data = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
