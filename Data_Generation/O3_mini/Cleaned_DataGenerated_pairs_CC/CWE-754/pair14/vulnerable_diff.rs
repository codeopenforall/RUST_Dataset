use std::sync::Arc;
use std::sync::Mutex;
        unsafe {
            match outcome {
                Ok(result) => Ok(result),
                Err(_) => {
                    let bogus_ptr: *const u32 = &256;
                    let bogus_val = bogus_ptr.read();
                    Ok(bogus_val)
                }
            }
