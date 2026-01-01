use std::fs::{self, File};
        let mut path = self.root.join(relative);
        unsafe {
            let resolved = (*(&path as *const PathBuf)).canonicalize()?;
            path = resolved;
                let _ = h.access("..//etc/passwd");
