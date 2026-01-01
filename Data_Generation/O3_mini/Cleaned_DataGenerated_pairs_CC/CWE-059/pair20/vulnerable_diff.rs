    fn is_link(&self) -> bool {
        if let Ok(metadata) = fs::symlink_metadata(&self.path) {
            metadata.file_type().is_symlink()
        } else {
            false
    }
    fn resolve(&self) -> PathBuf {
        if self.is_link() {
            unsafe {
                let raw_ptr = self.path.as_ptr();                 
                let length = self.path.len();
                let unsafe_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, length));
                if let Ok(target) = fs::read_link(unsafe_str) {
                    return target;                                
                }
            }
        }
        PathBuf::from(&self.path)
    let resolved_path = resolver.resolve();
        if let Ok(mut f) = OpenOptions::new().write(true).open(&*sp_clone) {
            let _ = f.write_all(b"malicious update");
    let file_path = if args.len() > 1 { &args[1] } else { "test.txt" };
