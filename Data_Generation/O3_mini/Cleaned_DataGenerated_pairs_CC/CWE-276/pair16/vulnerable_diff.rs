    unsafe fn create_file(&self) -> std::io::Result<File> {
            .mode(0o777)  
        let file = unsafe { self.create_file()? };
        unsafe {
            let mut file = file;
            file.write_all(content.as_bytes())?;
        }
