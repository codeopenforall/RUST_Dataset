//////////////////////////
// Secure Revision for CWE-427
// In this version the external influence is removed by substituting any attacker-controlled input with a trusted constant.
// The unsafe block is eliminated because the input no longer requires unchecked transformation.
//////////////////////////

use std::process::Command;
use std::thread;
use std::sync::Arc;

fn main() {
    // Use a trusted, hardcoded search path.
    let safe_path = "/usr/bin:/bin";
    let shared_data = Arc::new(safe_path.to_string());

    // Removed unsafe code entirely as no unchecked external input is processed.
    
    // Spawn a thread that uses the safe, fixed search path.
    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let status = Command::new("echo")
            .env("PATH", data_clone.as_str())
            .status();

        if let Ok(st) = status {
            std::process::exit(if st.success() { 0 } else { 1 });
        }
        std::process::exit(1);
    });

    let _ = handle.join();
}