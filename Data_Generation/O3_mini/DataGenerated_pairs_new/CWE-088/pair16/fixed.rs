use std::process::Command;

trait Executor {
    fn run(&self, input: &str) -> Result<String, String>;
}

struct CmdRunner;
impl CmdRunner {
    fn new() -> Self {
        CmdRunner
    }
}

impl Executor for CmdRunner {
    fn run(&self, input: &str) -> Result<String, String> {
        // By avoiding the shell entirely and passing the untrusted input
        // as a safe literal argument to the echo command, we prevent accidental interpretation
        // of metacharacters. This eliminates the OS command injection vulnerability.
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|e| format!("Execution failed: {}", e))?;
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
    }
}

fn main() {
    let runner = CmdRunner::new();
    let payload = "test; echo injected";
    match runner.run(payload) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}