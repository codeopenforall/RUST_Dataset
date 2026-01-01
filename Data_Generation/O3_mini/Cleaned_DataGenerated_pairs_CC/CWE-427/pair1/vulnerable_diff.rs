use std::ptr;
        let raw = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        unsafe {
            let mut derived = raw.clone();
            let ptr_str: *mut String = &mut derived;
            (*ptr_str).push_str("");
            AppConfig {
                search_path: derived,
            }
