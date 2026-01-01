use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;
struct SafeExecutor;
impl SafeExecutor {
    fn trusted_path() -> String {
        "/usr/bin:/bin".to_string()
    }
    fn launch() -> Result<String, String> {
        let safe_path = Self::trusted_path();
        let output = Command::new("echo")
            .env("PATH", safe_path)
            .arg("hello")
            .output();
        match output {
            Ok(result) => {
                let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                Ok(out_str)
            },
            Err(e) => Err(format!("Command execution failed: {:?}", e)),
        }
    }
}
fn main() {
    let modifier = thread::spawn(|| {
        for i in 0..5 {
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });
    modifier.join().unwrap();
    match SafeExecutor::launch() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}
