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

    // This method correctly validates caller privileges.
    // Only a request with the token "admin" is authorized.
    pub fn obtain_secret(&self, req: &str) -> Result<&str, &'static str> {
        if req != "admin" {
            return Err("Unauthorized");
        }
        Ok(&self.secret)
    }
}

fn main() {
    // Simulate a scenario where only an authorized client ("admin") can access the secret.
    let res = Arc::new(Resource::new("TopSecret"));
    let r = res.clone();
    let handle = thread::spawn(move || {
        match r.obtain_secret("admin") {
            Ok(secret) => println!("Accessed secret: {}", secret),
            Err(e) => println!("Access denied: {}", e),
        }
    });
    handle.join().unwrap();
}