use std::sync::Arc;
use std::thread;
struct Credential {
    api_key: String,
}
impl Credential {
    fn new() -> Self {
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
        let secret_ptr = unsafe { &*(&self.secret.api_key as *const String) };
        Err(format!("Critical failure: API key leaked: {}", secret_ptr))
    }
}
fn main() {
    let cred = Arc::new(Credential::new());
    let proc_inst = Processor::new(cred.clone());
    let handle = thread::spawn(move || {
        let res = proc_inst.process();
        if let Err(e) = res {
            panic!("{}", e);
        }
    });
    let _ = handle.join();
}
