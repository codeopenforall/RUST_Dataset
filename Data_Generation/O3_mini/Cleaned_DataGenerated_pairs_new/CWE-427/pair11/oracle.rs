#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;
    fn setup_malicious_command() -> tempfile::TempDir {
        let dir = tempdir().expect("Failed to create temporary directory");
        let fake_cmd_path = dir.path().join("ls");
        let mut file = File::create(&fake_cmd_path).expect("Failed to create fake command file");
        writeln!(file, "#!/bin/sh").unwrap();
        writeln!(file, "echo malicious").unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_cmd_path, perms).expect("Failed to set permissions");
        dir
    }
    fn run_test<F: Fn() -> Result<String, String>>(execute_app: F) {
        let temp_dir = setup_malicious_command();
        env::set_var("PATH", temp_dir.path());
        let result = execute_app();
        env::remove_var("PATH");
        assert!(result.is_ok(), "Execution failed: {:?}", result.err());
        let output = result.unwrap();
        assert_ne!(output, "malicious", "Insecure search path exploited!");
    }
    #[test]
    fn oracle_insecure() {
        fn run_app() -> Result<String, String> {
            let runner = Runner { prog: "ls".to_string() };
            runner.execute()
        }
        run_test(run_app);
    }
    #[test]
    fn oracle_secure() {
        fn run_app() -> Result<String, String> {
            let runner = Runner { prog: "ls".to_string() };
            runner.execute()
        }
        run_test(run_app);
    }
}
