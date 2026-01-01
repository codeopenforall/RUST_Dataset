#[cfg(test)]
mod tests {
    use super::Handler;
    use std::fs::{write, remove_file, remove_dir_all, canonicalize};
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;
    use std::env;
    fn setup_env(temp_dir: &Path) -> (String, String, String) {
        let safe_path = temp_dir.join("safe.txt");
        let malicious_path = temp_dir.join("malicious.txt");
        let link_path = temp_dir.join("link.txt");
        write(&safe_path, "safe content").expect("Failed to write safe file");
        write(&malicious_path, "malicious content").expect("Failed to write malicious file");
        let _ = remove_file(&link_path);
        symlink(&safe_path, &link_path).expect("Failed to create symlink");
        (
            safe_path.to_str().unwrap().to_owned(),
            malicious_path.to_str().unwrap().to_owned(),
            link_path.to_str().unwrap().to_owned(),
        )
    }
    #[test]
    fn test_toctou_prevention() {
        let temp_dir = env::current_dir().unwrap().join("temp_test_dir");
        let _ = std::fs::create_dir(&temp_dir);
        let (_safe, malicious, link) = setup_env(&temp_dir);
        let link_clone = link.clone();
        let malicious_clone = malicious.clone();
        let swap_thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = remove_file(&link_clone);
            symlink(&malicious_clone, &link_clone).expect("Failed to swap symlink");
        });
        let result = Handler::execute(&link);
        swap_thread.join().unwrap();
        let _ = remove_dir_all(&temp_dir);
        match result {
            Ok(content) => assert_eq!(content, "safe content", "Unexpected file content read"),
            Err(e) => panic!("Operation failed with error: {}", e),
        }
    }
}
