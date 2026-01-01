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
        // The design below unsafely constructs a shell command by interpolating the untrusted input.
        // Using "sh -c" the formatted string is interpreted by the shell,
        // which makes it vulnerable to OS command injections if input contains metacharacters.
        unsafe {
            let command = format!("echo {}", input);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .map_err(|e| format!("Execution failed: {}", e))?;
            let result = String::from_utf8_lossy(&output.stdout).into_owned();
            Ok(result)
        }
    }
}

fn main() {
    let runner = CmdRunner::new();
    // Input deliberately crafted to include shell metacharacters.
    let injection = "test; echo injected";
    match runner.run(injection) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}