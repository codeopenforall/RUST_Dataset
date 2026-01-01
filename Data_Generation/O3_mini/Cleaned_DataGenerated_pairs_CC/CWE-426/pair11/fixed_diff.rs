use std::env;
use std::path::{Path, PathBuf};
use std::fs;
        let exe_path = env::current_exe().map_err(|e| format!("Current exe error: {}", e))?;
        let parent_dir = exe_path.parent().ok_or_else(|| "Failed to determine exe directory".to_string())?;
        let safe_path: PathBuf = [parent_dir, Path::new("trusted"), Path::new(&self.command)].iter().collect();
        let safe_str = safe_path.to_str().ok_or_else(|| "Invalid path string".to_string())?;
            let mut child = Command::new(safe_str)
