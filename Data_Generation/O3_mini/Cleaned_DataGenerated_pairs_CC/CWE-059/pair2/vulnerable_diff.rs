use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
        let metadata = fs::metadata(path)?;
        unsafe {
            let dummy_ptr: *const u8 = &metadata as *const _ as *const u8;
            let _ = *dummy_ptr; 
        }
        thread::sleep(Duration::from_millis(100));
        let content = fs::read_to_string(path)?;
