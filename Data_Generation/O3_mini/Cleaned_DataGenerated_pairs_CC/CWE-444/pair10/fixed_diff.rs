    fn parse_headers(&self) -> Option<usize> {
        self.raw.find("\r\n\r\n").map(|i| i + 4)
        let body_start = self.parse_headers()?;
        Some(&self.raw.as_bytes()[..body_start])
            return false;
