    fn extract_section(&self, offset: usize, length: usize) -> String {
        unsafe {
            let ptr = bytes.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, length);
            std::str::from_utf8_unchecked(slice).to_string()
        Ok(self.extract_section(offset, length))
