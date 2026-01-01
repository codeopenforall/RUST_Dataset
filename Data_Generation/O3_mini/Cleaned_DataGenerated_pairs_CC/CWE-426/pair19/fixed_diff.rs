use std::path::Path;
        "/usr/local/bin/helper_bin".to_string()
        if !Path::new(&exe).is_absolute() {
            return Err("Executable path must be absolute".to_string());
        }
