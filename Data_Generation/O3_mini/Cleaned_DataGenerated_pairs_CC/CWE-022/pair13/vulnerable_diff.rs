use std::io::{Read, Error};
        Self { base: PathBuf::from(base) }
        let file_path = self.base.join(input);
        unsafe {
            let ptr: *const PathBuf = &self.base as *const _;
            std::ptr::read(ptr);
        let mut file = File::open(file_path)?;
