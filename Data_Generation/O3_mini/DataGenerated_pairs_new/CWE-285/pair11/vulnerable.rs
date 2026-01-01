use std::sync::Arc;
use std::thread;

struct Resource {
    secret: String,
}

impl Resource {
    pub fn new(secret: &str) -> Self {
        Resource {
            secret: secret.to_string(),
        }
    }

    // This method is unsafe and improperly implements authorization.
    // It ignores the provided request identifier and always returns the secret.
    pub unsafe fn obtain_secret(&self, _req: &str) -> Result<&str, &'static str> {
        // Vulnerability: missing proper access control check (CWE-285).
        Ok(&self.secret)
    }
}

fn main() {
    // Simulate a scenario where an unauthorized client ("user") is requesting access.
    let res = Arc::new(Resource::new("TopSecret"));
    let r = res.clone();
    let handle = thread::spawn(move || {
        unsafe {
            match r.obtain_secret("user") {
                Ok(secret) => println!("Accessed secret: {}", secret),
                Err(e) => println!("Access denied: {}", e),
            }
        }
    });
    handle.join().unwrap();
}