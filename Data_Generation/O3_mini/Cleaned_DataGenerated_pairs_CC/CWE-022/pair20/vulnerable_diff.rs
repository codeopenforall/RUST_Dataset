use std::fs::File;
    fn new(base: PathBuf) -> Self {
        Self { base }
        let content = unsafe {
            let mut file = File::open(&target).ok()?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).ok()?;
            String::from_utf8_unchecked(buf)
        };
        Some(content)
        None => println!("Failed to load file."),
