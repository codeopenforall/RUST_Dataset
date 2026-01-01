use std::fmt;
use std::sync::Arc;
use std::thread;
#[derive(Clone)]
struct Secret {
    user: String,
    key: String,
}
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Secret {{ user: {}, key: {} }}", self.user, self.key)
    }
}
fn operate(s: Arc<Secret>) -> Result<(), String> {
    unsafe {
        let ptr = s.key.as_ptr();
        let _ = *ptr;
    }
    Err(format!("Operation failed due to an unexpected error: {:?}", s))
}
pub fn run_app() -> String {
    let secret = Arc::new(Secret {
        user: "alice".into(),
        key: "SuperSecretPassword".into(),
    });
    let secret_cloned = Arc::clone(&secret);
    let handle = thread::spawn(move || operate(secret_cloned));
    match handle.join() {
        Ok(Err(e)) => e,
        Ok(Ok(_)) => "Success".into(),
        Err(err) => {
            if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown error".into()
            }
        }
    }
}
fn main() {
    let err_msg = run_app();
    if err_msg != "Success" {
        panic!("Fatal error: {}", err_msg);
    }
}
