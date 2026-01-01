    fn extract_section(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        if offset.checked_add(length).filter(|&end| end <= bytes.len()).is_none() {
            return Err("Invalid bounds");
        let slice = &self.content[offset..offset + length];
        Ok(slice.to_string())
        self.extract_section(offset, length)
