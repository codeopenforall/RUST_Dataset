////////////////////////////////////////////////////////////
// Secure Example for CWE-427: Uncontrolled Search Path Element Fix
// This corrected version validates the input obtained from the environment variable "MALICIOUS_PATH".
// If the supplied value is not within an acceptable safe set (i.e. does not start with "/usr/bin"), then a safe default
// is used instead to prevent unintended modification of the search path.
////////////////////////////////////////////////////////////

use std::env;
use std::process::Command;
use std::thread;

trait Executor {
    fn launch(&self) -> Result<(), std::io::Error>;
}

struct Application;

impl Application {
    // Securely resolve the PATH value. Only allow values that begin with "/usr/bin".
    // If the input fails the check, a safe default ("/usr/bin:/bin") is used.
    fn resolve_path(&self) -> String {
        let custom = env::var("MALICIOUS_PATH").unwrap_or_default();
        if !custom.is_empty() && custom.starts_with("/usr/bin") {
            // Use safe conversion; this will check UTF-8 validity.
            if let Ok(valid) = std::str::from_utf8(custom.as_bytes()) {
                return valid.to_string();
            }
        }
        // Fallback to a safe default PATH.
        String::from("/usr/bin:/bin")
    }
}

impl Executor for Application {
    fn launch(&self) -> Result<(), std::io::Error> {
        let chosen_path = self.resolve_path();
        let mut cmd = Command::new("ls");
        // Set the environment PATH to the verified value.
        cmd.env("PATH", chosen_path);
        // Launch the command in a separate thread.
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