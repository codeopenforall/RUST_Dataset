use std::path::{Component, Path, PathBuf};
        let candidate = self.base.join(input);
        for comp in candidate.components() {
            if let Component::ParentDir = comp {
                return Err("Path traversal attempt detected".into());
            }
        Ok(candidate)
