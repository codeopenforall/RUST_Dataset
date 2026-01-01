use std::io::{Error, ErrorKind};
        let joined = self.base.join(filename);
        let canon_base = fs::canonicalize(&self.base)?;
        let canon_target = fs::canonicalize(&joined)?;
        if !canon_target.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
            let bytes = canon_target.to_str().unwrap().as_bytes();
        fs::read_to_string(canon_target)
