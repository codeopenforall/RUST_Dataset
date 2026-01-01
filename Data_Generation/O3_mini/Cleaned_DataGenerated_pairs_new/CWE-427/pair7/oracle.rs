#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use super::spawn_process;
    use super::Config;
    #[test]
    fn test_secure_path_enforcement() {
        env::set_var("PATH", "malicious");
        let cfg = Config::new("printenv", 10);
        let output = spawn_process(cfg).expect("Process execution failed");
        let output_trimmed = output.trim();
        let expected = "/usr/bin:/bin";
        assert_eq!(output_trimmed, expected, "The effective PATH was not sanitized as expected");
    }
}
