use std::io::{Error, ErrorKind, Read, Result};
        let candidate = self.root.join(req);
        let real_candidate = candidate.canonicalize()?;
        let safe_root = self.root.canonicalize()?;
        if !real_candidate.starts_with(&safe_root) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
            let mut file = File::open(&real_candidate)?;
