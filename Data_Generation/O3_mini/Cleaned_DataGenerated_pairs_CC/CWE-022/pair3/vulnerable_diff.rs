use std::io::{Read, Error};
use std::path::PathBuf;
        let mut full = self.base.clone();
        full.push(input); 
        let path_str = full.to_str().ok_or_else(|| Error::from_raw_os_error(22))?;
        unsafe {
            let raw_ptr = path_str.as_ptr();
            let len = path_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let manipulated = std::str::from_utf8(slice).map_err(|_| Error::from_raw_os_error(22))?;
            full = PathBuf::from(manipulated);
        let path_clone = full.clone();
    let filename = "../etc/passwd";
