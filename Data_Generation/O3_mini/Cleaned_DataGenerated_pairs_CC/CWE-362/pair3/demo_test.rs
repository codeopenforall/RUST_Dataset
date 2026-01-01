#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // The test harness executes the compiled binary and inspects its output.
    #[test]
    fn concurrent_counter_test() {
        // Determine the path to the compiled binary.
        let exe_path = std::env::args().next().expect("Cannot determine binary path");

        let output = Command::new(exe_path)
            .output()
            .expect("Failed to execute process");

        // If the process panicked (non-zero exit code), then the invariant was violated.
        assert!(
            output.status.success(),
            "Process did not complete successfully; may be due to race condition."
        );

        let stdout_str = str::from_utf8(&output.stdout).expect("Output not valid UTF-8");
        // Expect the printed final counter to match 40000 (NUM_THREADS * ITER).
        assert!(
            stdout_str.contains("Final counter: 40000"),
            "Output did not contain expected counter value. Got: {}",
            stdout_str
        );
    }
}