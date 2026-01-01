use std::fs::{File, canonicalize};
    allowed: PathBuf,
    fn new(mut base: PathBuf) -> Self {
        let allowed = canonicalize(&base).expect("Failed to canonicalize base path");
        Self { base, allowed }
        let target_canon = canonicalize(&target).ok()?;
        if !target_canon.starts_with(&self.allowed) {
            return None;
        }
        let mut file = File::open(&target_canon).ok()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).ok()?;
        String::from_utf8(buf).ok()
        None => println!("Failed to load file or invalid access."),
