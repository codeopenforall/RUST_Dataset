/////////////////////////////////////////////////////////////
// Vulnerable Version
/////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Holder {
    secret: String,
}

fn process_data(holder: Arc<Holder>) -> Result<(), String> {
    // Simulate complex processing that erroneously exposes sensitive information.
    //
    // An unsafe block is used to manipulate pointers to the secret,
    // and the secret is inadvertently embedded in an error message.
    unsafe {
        let secret_ptr: *const u8 = holder.secret.as_ptr();
        let leaked = std::slice::from_raw_parts(secret_ptr, holder.secret.len());
        // Intentionally skip UTF-8 checks: real-world code might do this for performance.
        let leaked_str = std::str::from_utf8_unchecked(leaked);
        // Simulate a failure condition that logs the sensitive data.
        if leaked_str.len() > 0 {
            // Vulnerability: Sensitive secret information is directly included in the error output.
            return Err(format!("Operation failed on data: {}", leaked_str));
        }
    }
    Ok(())
}

fn main() {
    let holder = Arc::new(Holder {
        secret: "SuperSecretPassword1234".to_owned(),
    });
    let holder_clone = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        let _ = process_data(holder_clone);
    });
    let _ = handle.join();
}