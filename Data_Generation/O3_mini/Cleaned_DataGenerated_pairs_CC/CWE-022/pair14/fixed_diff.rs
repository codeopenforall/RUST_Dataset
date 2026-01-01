use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
        let joined = self.base.join(input);
        let canon_joined = joined.canonicalize()?;
        let canon_base = self.base.canonicalize()?;
        if !canon_joined.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Path traversal attempt detected"));
        }
            let s = canon_joined.as_os_str().to_str().unwrap();
        fs::read_to_string(canon_joined)
