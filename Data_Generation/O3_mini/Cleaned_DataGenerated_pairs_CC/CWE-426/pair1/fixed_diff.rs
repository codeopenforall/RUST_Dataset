use std::path::Path;
        let path = Path::new(cmd);
        if path.is_absolute() {
            cmd.to_string()
        } else {
            format!("/usr/bin/{}", cmd)
        }
