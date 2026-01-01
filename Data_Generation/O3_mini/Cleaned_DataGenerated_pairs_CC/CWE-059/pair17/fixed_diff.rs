use std::fs::{File, symlink_metadata};
use std::io::{Read, Result, Error, ErrorKind};
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
        let meta = symlink_metadata(path)?;
        #[cfg(unix)]
        {
            if meta.file_type().is_symlink() {
                return Err(Error::new(ErrorKind::Other, "Symbolic links are disallowed"));
            }
        }
