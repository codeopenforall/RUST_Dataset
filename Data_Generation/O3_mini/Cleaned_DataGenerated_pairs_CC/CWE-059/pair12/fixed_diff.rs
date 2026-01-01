use std::path::{Path, PathBuf, Component};
    base: PathBuf,
        Manager { base: PathBuf::from(base) }
    }
    fn normalize_path(&self, path: &Path) -> Option<PathBuf> {
        let mut normalized = PathBuf::new();
        for comp in path.components() {
            match comp {
                Component::ParentDir => return None, 
                Component::CurDir => continue,
                other => normalized.push(other.as_os_str()),
            }
        }
        Some(normalized)
        let candidate = self.base.join(input);
        let resolved = self.normalize_path(&candidate)
            .ok_or("Invalid path: traversal attempt detected")?;
        if !resolved.starts_with(&self.base) {
            return Err("Access denied: outside allowed directory");
        }
        if input.contains("..") {
            Err("Access denied: traversal detected")
        println!("Thread result: {:?}", result);
