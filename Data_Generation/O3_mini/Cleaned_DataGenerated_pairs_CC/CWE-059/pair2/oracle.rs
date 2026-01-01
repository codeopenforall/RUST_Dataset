#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::thread;
    use std::time::Duration;
    use super::FileProcessor;
    #[test]
    fn test_symlink_race_condition() -> io::Result<()> {
        let tmp_dir = std::env::temp_dir().join("rust_symlink_test");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir)?;
        let safe_file: PathBuf = tmp_dir.join("safe.txt");
        let malicious_file: PathBuf = tmp_dir.join("malicious.txt");
        let link_file: PathBuf = tmp_dir.join("target.txt");
        fs::write(&safe_file, "safe")?;
        fs::write(&malicious_file, "bad")?;
        let _ = fs::remove_file(&link_file);
        symlink(&safe_file, &link_file)?;
        let processor = FileProcessor;
        let link_clone = link_file.clone();
        let malicious_clone = malicious_file.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = fs::remove_file(&link_clone);
            symlink(&malicious_clone, &link_clone).expect("Failed to update symlink");
        });
        let result = processor.process(link_file.to_str().unwrap())?;
        handle.join().unwrap();
        assert_eq!(result, "safe", "File content mismatch due to symlink race condition");
        Ok(())
    }
}
