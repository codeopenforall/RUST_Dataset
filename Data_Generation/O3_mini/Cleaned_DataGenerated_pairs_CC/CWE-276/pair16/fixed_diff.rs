    fn create_file(&self) -> std::io::Result<File> {
            .mode(0o600)
        let mut file = self.create_file()?;
        file.write_all(content.as_bytes())?;
