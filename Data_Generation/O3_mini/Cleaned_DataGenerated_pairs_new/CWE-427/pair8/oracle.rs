#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::thread;
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
        env::set_var("MAL_INPUT", "/tmp/malicious");
        let exec = Executor{};
        let ret = exec.launch(&env::var("MAL_INPUT").unwrap());
        assert_eq!(ret, 0, "Test oracle: The execution failed due to uncontrolled search path element");
    }
}
