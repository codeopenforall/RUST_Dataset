        let expected = format!("https://{}", self.config);
        origin.contains(&expected)
