use std::time::Duration;
    unsafe {
        if GLOBAL_RESOURCE.is_null() {                   
            thread::sleep(Duration::from_millis(50));    
            GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
        }
        GLOBAL_RESOURCE
    }
