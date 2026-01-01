#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::thread;

    // Duplicate of the vulnerable Executor to test behavior.
    struct Executor;

    impl Executor {
        fn launch(&self, path: &str) -> i32 {
            if path.contains("/tmp/malicious") {
                return 1;
            }
            let mut child = Command::new("sh")
                .arg("-c")
                .arg("echo Execution")
                .env("PATH", path)
                .spawn()
                .expect("failed to spawn process");
            let status = child.wait().expect("process wasn't running");
            status.code().unwrap_or(-1)
        }
    }

    #[test]
    fn test_executor_behavior() {
        // For the vulnerable variant, setting a malicious PATH should cause a non-zero exit code.
        env::set_var("MAL_INPUT", "/tmp/malicious");
        let exec = Executor{};
        let ret = exec.launch(&env::var("MAL_INPUT").unwrap());
        // The vulnerable version returns 1 when a malicious search path is used.
        // The fixed version would ignore the untrusted input and return 0.
        assert_eq!(ret, 0, "Test oracle: The execution failed due to uncontrolled search path element");
    }
}