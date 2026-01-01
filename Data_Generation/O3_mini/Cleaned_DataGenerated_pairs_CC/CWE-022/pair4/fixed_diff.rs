use std::fs::{File, canonicalize};
        let base_dir = canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        FileHandler { base_dir }
        let candidate = self.base_dir.join(input);
        let candidate_canon = canonicalize(&candidate)?;
        if !candidate_canon.starts_with(&self.base_dir) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access outside the permitted directory is denied",
            ));
        }
        let mut file = File::open(&candidate_canon)?;
