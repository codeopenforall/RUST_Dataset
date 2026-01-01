use std::fs::{self, File};
use std::path::{Path, PathBuf};
        let path = Path::new(input);
        let canonical = fs::canonicalize(path)
            .map_err(|e| format!("Canonicalization error: {}", e))?;
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let safe_dir = current_dir.join("safe");
        if !canonical.starts_with(&safe_dir) {
            return Err("Access denied: path is outside the safe directory".to_string());
        canonical
            .into_os_string()
            .into_string()
            .map_err(|_| "Failed to convert path".to_string())
    let file = File::open(&resolved_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .map_err(|e| format!("Error reading file: {}", e))?;
    Ok(content)
    let safe_dir = std::env::current_dir().unwrap().join("safe");
    fs::create_dir_all(&safe_dir).unwrap();
        safe_dir.join("test_file.txt").into_os_string().into_string().unwrap()
