use std::time::Duration;
    unsafe {
        let ptr = data.as_ptr();
        println!("Transmitting from pointer: {:?}", ptr);
        let leaked = std::slice::from_raw_parts(ptr, data.len());
        println!("Logged Data (vulnerable): {:?}", leaked);
    }
        unsafe {
            let ptr = shared_config.sensitive.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, shared_config.sensitive.len());
            let extracted = String::from_utf8_lossy(slice);
            send(&extracted)
        }
