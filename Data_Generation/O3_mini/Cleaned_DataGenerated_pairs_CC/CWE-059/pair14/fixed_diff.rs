use std::fs::File;
        let path = self.root.join(relative).canonicalize()?;
        if !path.starts_with(&self.root) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access denied: resolved path is outside the allowed directory",
            ));
                let _ = h.access("sample.txt");
