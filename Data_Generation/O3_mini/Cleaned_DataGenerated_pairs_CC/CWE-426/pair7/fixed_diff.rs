use std::path::PathBuf;
        let mut abs_path = std::env::current_dir().expect("Failed to get current directory");
        abs_path.push("plugin.so");
        Self { command: abs_path.to_string_lossy().into_owned() }
