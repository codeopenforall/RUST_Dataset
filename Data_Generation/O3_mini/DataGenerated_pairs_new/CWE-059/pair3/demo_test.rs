#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    // Assuming process_file is the function under test in the library.
    use crate::process_file;

    #[test]
    fn test_symlink_protection() -> io::Result<()> {
        // Setup: Create a target file with dummy (but sensitive) content.
        let target_path = "target.txt";
        let mut target = File::create(target_path)?;
        writeln!(target, "Sensitive Information")?;
        target.sync_all()?;

        // Create a symlink that points to the target file.
        let link_path = "link.txt";
        if Path::new(link_path).exists() {
            fs::remove_file(link_path)?;
        }
        #[cfg(unix)]
        symlink(target_path, link_path)?;
        #[cfg(not(unix))]
        {
            // Skip test if symlinks are not supported on this platform.
            return Ok(());
        }

        // Call the file processing function with the symlink.
        let result = process_file(link_path);

        // For secure behavior, symlink usage must be rejected.
        assert!(
            result.is_err(), 
            "Test failed: Symlink was incorrectly accepted. Vulnerability present."
        );

        // Cleanup the created files.
        fs::remove_file(target_path)?;
        fs::remove_file(link_path)?;
        Ok(())
    }
}