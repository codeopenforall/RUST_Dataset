#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use crate::process_file;
    #[test]
    fn test_symlink_protection() -> io::Result<()> {
        let target_path = "target.txt";
        let mut target = File::create(target_path)?;
        writeln!(target, "Sensitive Information")?;
        target.sync_all()?;
        let link_path = "link.txt";
        if Path::new(link_path).exists() {
            fs::remove_file(link_path)?;
        }
        #[cfg(unix)]
        symlink(target_path, link_path)?;
        #[cfg(not(unix))]
        {
            return Ok(());
        }
        let result = process_file(link_path);
        assert!(
            result.is_err(), 
            "Test failed: Symlink was incorrectly accepted. Vulnerability present."
        );
        fs::remove_file(target_path)?;
        fs::remove_file(link_path)?;
        Ok(())
    }
}
