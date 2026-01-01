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
    let injection = "test; echo injected";
    match runner.run(injection) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
