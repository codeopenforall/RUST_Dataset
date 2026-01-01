use std::path::{Path, PathBuf};
        let canonical_full = fs::canonicalize(&full_path)?;
        let canonical_root = fs::canonicalize(&self.root)?;
        if !canonical_full.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Traversal attempt detected."));
        }
        let contents = fs::read_to_string(&canonical_full)?;
