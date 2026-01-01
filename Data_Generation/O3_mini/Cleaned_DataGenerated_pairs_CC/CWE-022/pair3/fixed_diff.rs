use std::io::{Read, Error, ErrorKind};
use std::path::{PathBuf, Component};
        let user_path = PathBuf::from(input);
        for comp in user_path.components() {
            if let Component::ParentDir = comp {
                return Err(Error::new(ErrorKind::InvalidInput, "Path traversal is not allowed"));
            }
        let joined = self.base.join(&user_path)
            .canonicalize()?;
        if !joined.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::InvalidInput, "Resolved path is outside the designated directory"));
        }
        let joined_str = joined.to_str().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid path conversion"))?;
        let mut safe_path = PathBuf::new();
        unsafe {
            let raw_ptr = joined_str.as_ptr();
            let len = joined_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let validated = std::str::from_utf8(slice).map_err(|_| Error::new(ErrorKind::Other, "UTF8 error"))?;
            safe_path = PathBuf::from(validated);
        }
        let path_clone = safe_path.clone();
    let filename = "allowed.txt";
