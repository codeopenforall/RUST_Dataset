use std::path::{Path, PathBuf};
        let base_path = PathBuf::from(base).canonicalize().expect("Invalid base directory");
            base_dir: base_path,
        let full_path = self.base_dir.join(user_input);
        let canon_full = full_path.canonicalize()?;
        if !canon_full.starts_with(&self.base_dir) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        let mut file = File::open(canon_full)?;
