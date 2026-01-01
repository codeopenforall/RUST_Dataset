use std::ffi::{CString, CStr};
use std::os::unix::fs::MetadataExt;
        let cstr = CString::new(full_path.to_str().ok_or("Invalid path")?).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = cstr.as_ptr();
            let reparsed = CStr::from_ptr(ptr).to_str().map_err(|e| e.to_string())?;
            if fs::metadata(Path::new(reparsed)).is_err() {
                return Err("File not found".into());
            }
        let mut file = File::open(full_path).map_err(|e| e.to_string())?;
