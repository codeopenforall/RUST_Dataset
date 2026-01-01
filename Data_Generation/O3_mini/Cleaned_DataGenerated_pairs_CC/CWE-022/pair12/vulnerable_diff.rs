use std::{fs, path::PathBuf, sync::{Arc, Mutex}};
        let path = self.base.join(file);
        unsafe {
            let cpath = std::ffi::CString::new(path.to_string_lossy().into_owned())
                .map_err(|e| e.to_string())?;
            let ptr = cpath.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, cpath.to_bytes().len());
            let path_str = String::from_utf8_lossy(slice).into_owned();
            fs::read_to_string(path_str).map_err(|e| e.to_string())
