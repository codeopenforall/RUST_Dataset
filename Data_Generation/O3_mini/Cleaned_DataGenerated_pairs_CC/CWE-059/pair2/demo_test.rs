//////////////////////////////////////////////
// Test Oracle Function for Symlink Handling  //
// (Fails for vulnerable, passes for secure) //
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::thread;
    use std::time::Duration;

    // Import the FileProcessor from the implementation under test.
    // For the vulnerable build, this test should fail by reading unexpected content.
    // For the secure build, the test should pass by consistently retrieving the "safe" content.
    use super::FileProcessor;

    #[test]
    fn test_symlink_race_condition() -> io::Result<()> {
        // Setup temporary directory and file paths.
        let tmp_dir = std::env::temp_dir().join("rust_symlink_test");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir)?;

        let safe_file: PathBuf = tmp_dir.join("safe.txt");
        let malicious_file: PathBuf = tmp_dir.join("malicious.txt");
        let link_file: PathBuf = tmp_dir.join("target.txt");

        // Write distinct content to the safe and malicious files.
        fs::write(&safe_file, "safe")?;
        fs::write(&malicious_file, "bad")?;

        // Ensure the symlink initially points to the safe file.
        let _ = fs::remove_file(&link_file);
        symlink(&safe_file, &link_file)?;

        let processor = FileProcessor;

        // Spawn a thread that, after a brief delay, swaps the symlink to point to the malicious file.
        let link_clone = link_file.clone();
        let malicious_clone = malicious_file.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = fs::remove_file(&link_clone);
            symlink(&malicious_clone, &link_clone).expect("Failed to update symlink");
        });

        // Process the file via the provided symlink.
        let result = processor.process(link_file.to_str().unwrap())?;
        handle.join().unwrap();

        // In a secure implementation, the opened file remains "safe"; a TOCTOU vulnerability
        // in the vulnerable version could result in reading "bad".
        assert_eq!(result, "safe", "File content mismatch due to symlink race condition");
        Ok(())
    }
}