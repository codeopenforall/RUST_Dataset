use std::path::{PathBuf};
            base_dir: PathBuf::from(base),
        let safe_str: &str;
        unsafe {
            safe_str = std::str::from_utf8_unchecked(user_input.as_bytes());
        let mut full_path = self.base_dir.clone();
        full_path = full_path.join(safe_str);
        let mut file = File::open(full_path)?;
