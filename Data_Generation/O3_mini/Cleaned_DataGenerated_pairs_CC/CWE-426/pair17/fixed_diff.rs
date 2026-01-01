use std::path::Path;
        let p = Path::new(&self.path);
        if !p.is_absolute() {
            return Err("Relative paths are rejected".to_owned());
        }
        let abs_path = p.canonicalize().map_err(|e| e.to_string())?;
            let status = Command::new(abs_path)
    let launcher = Arc::new(Launcher { path: "/usr/bin/vulnerable_app".to_owned() });
