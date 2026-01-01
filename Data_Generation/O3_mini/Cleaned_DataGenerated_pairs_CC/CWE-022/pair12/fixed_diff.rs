use std::{fs, path::{Path, PathBuf}, sync::{Arc, Mutex}};
        let candidate = self.base.join(file);
        let base_canonical = fs::canonicalize(&self.base).map_err(|e| e.to_string())?;
        let candidate_canonical = fs::canonicalize(&candidate).map_err(|e| e.to_string())?;
        if !candidate_canonical.starts_with(&base_canonical) {
            return Err("Access to files outside permitted directory is not allowed".to_owned());
        fs::read_to_string(candidate).map_err(|e| e.to_string())
