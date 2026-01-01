//////////////// Vulnerable Code Example ////////////////////
use std::sync::Arc;
use std::thread;

struct Credential {
    api_key: String,
}

impl Credential {
    fn new() -> Self {
        // In a real system, this would be loaded securely.
        Self { api_key: "SuperSecretAPIKey123".to_owned() }
    }
}

struct Processor {
    secret: Arc<Credential>,
}

impl Processor {
    fn new(secret: Arc<Credential>) -> Self {
        Self { secret }
    }

    fn process(&self) -> Result<(), String> {
        // Unsafe block used to simulate a low-level operation.
        // Vulnerability: The sensitive secret is directly included in error information.
        let secret_ptr = unsafe { &*(&self.secret.api_key as *const String) };
        // Returning an error that includes the sensitive API key.
        Err(format!("Critical failure: API key leaked: {}", secret_ptr))
    }
}

fn main() {
    let cred = Arc::new(Credential::new());
    let proc_inst = Processor::new(cred.clone());
    let handle = thread::spawn(move || {
        let res = proc_inst.process();
        // Panicking with the error message that leaks the secret.
        if let Err(e) = res {
            panic!("{}", e);
        }
    });
    let _ = handle.join();
}