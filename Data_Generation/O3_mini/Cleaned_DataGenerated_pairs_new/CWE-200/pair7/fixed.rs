use std::sync::Arc;
use std::thread;
struct Holder {
    secret: String,
}
fn process_data(holder: Arc<Holder>) -> Result<(), String> {
    if holder.secret.len() > 0 {
        return Err("Operation failed due to processing error".to_owned());
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
