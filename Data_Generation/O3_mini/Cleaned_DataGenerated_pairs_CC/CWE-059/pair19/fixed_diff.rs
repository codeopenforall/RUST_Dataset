use std::io::{self, ErrorKind};
        if !canon.starts_with(&self.root) {
            return Err(io::Error::new(
                ErrorKind::PermissionDenied,
                "Access denied: resolved path lies outside the allowed directory",
            ));
