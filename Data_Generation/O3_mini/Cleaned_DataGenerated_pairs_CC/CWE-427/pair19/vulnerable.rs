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
        unsafe {
            let ptr = custom.as_ptr();
            let len = custom.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice).to_string()
        }
    }
}
impl Executor for Application {
    fn launch(&self) -> Result<(), std::io::Error> {
        let chosen_path = self.resolve_path();
        let mut cmd = Command::new("ls");
        if !chosen_path.is_empty() {
            cmd.env("PATH", chosen_path);
        }
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
