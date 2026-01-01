use std::io::{Read, Error, ErrorKind};
        let canon_base = std::fs::canonicalize(&base)
            .expect("Failed to canonicalize the base directory");
        Self { base: canon_base }
        let joined = self.base.join(input);
        let canon = std::fs::canonicalize(&joined)?;
        if !canon.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Disallowed path traversal attempt"));
        let mut file = File::open(canon)?;
