use std::io::{self, Write, Read};
        unsafe {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o777)
                .open(&path)?;
            let raw_ptr = &file as *const File as *const u8;
            let _ = *raw_ptr; 
            drop(file);
        }
    let test_path = "/tmp/insecure_resource.txt";
