#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File, remove_file};
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    use std::path::PathBuf;
    pub fn run_oracle(process: fn(&str) -> std::io::Result<String>) {
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push("oracle_test");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir(&tmp_dir).unwrap();
        let victim: PathBuf = tmp_dir.join("victim.txt");
        let attacker: PathBuf = tmp_dir.join("attack.txt");
        {
            let mut file = File::create(&victim).unwrap();
            write!(file, "safe").unwrap();
        }
        {
            let mut file = File::create(&attacker).unwrap();
            write!(file, "attack").unwrap();
        }
        let victim_clone = victim.clone();
        let attacker_clone = attacker.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(30));
            let _ = remove_file(&victim_clone);
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&attacker_clone, &victim_clone).unwrap();
            }
            #[cfg(windows)]
            {
                std::os::windows::fs::symlink_file(&attacker_clone, &victim_clone).unwrap();
            }
        });
        let result = process(victim.to_str().unwrap());
        match result {
            Ok(content) => {
                assert!(content == "safe", "Oracle: Vulnerability present, read modified content: {}", content);
            },
            Err(e) => {
                assert!(e.to_string().contains("TOCTOU"), "Oracle: Unexpected error: {}", e);
            }
        }
    }
    #[test]
    fn test_oracle_with_function() {
        run_oracle(process_file);
    }
}
