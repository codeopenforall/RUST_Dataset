use std::path::{Path, PathBuf};
        let canonical_base = std::fs::canonicalize(base).expect("Invalid base directory");
        Loader { base: canonical_base }
    }
    fn is_allowed(&self, candidate: &Path) -> bool {
        if let Ok(canonical_candidate) = candidate.canonicalize() {
            canonical_candidate.starts_with(&self.base)
        } else {
            false
        }
        if !self.is_allowed(&full_path) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal detected",
            ));
        }
        let part = if size >= 2 { size / 2 } else { size };
                        *ptr.add(j) = 0;
