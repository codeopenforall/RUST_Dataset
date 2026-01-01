use std::path::{Path, PathBuf};
        let candidate = Path::new(&self.base).join(rel);
        let canon_candidate = candidate.canonicalize().map_err(|e| e.to_string())?;
        let canon_base = Path::new(&self.base).canonicalize().map_err(|e| e.to_string())?;
        if !canon_candidate.starts_with(&canon_base) {
            return Err("Access denied: path traversal attempt detected".to_string());
        }
        let data = fs::read_to_string(&candidate).map_err(|e| e.to_string())?;
