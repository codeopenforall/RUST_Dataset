use std::env;
use std::process::Command;
use std::thread;
trait Executor {
    fn launch(&self) -> Result<(), std::io::Error>;
}
struct Application;
impl Application {
    fn resolve_path(&self) -> String {
        let custom = env::var("MALICIOUS_PATH").unwrap_or_default();
        if !custom.is_empty() && custom.starts_with("/usr/bin") {
            if let Ok(valid) = std::str::from_utf8(custom.as_bytes()) {
                return valid.to_string();
            }
        }
        String::from("/usr/bin:/bin")
    }
}
impl Executor for Application {
    fn launch(&self) -> Result<(), std::io::Error> {
        let chosen_path = self.resolve_path();
        let mut cmd = Command::new("ls");
        cmd.env("PATH", chosen_path);
        let handler = thread::spawn(move || {
            cmd.output().expect("failed to run process")
        });
        handler.join().unwrap();
        Ok(())
    }
}
fn main() {
    let app = Application;
    if let Err(e) = app.launch() {
        eprintln!("Error occurred: {:?}", e);
    }
}
