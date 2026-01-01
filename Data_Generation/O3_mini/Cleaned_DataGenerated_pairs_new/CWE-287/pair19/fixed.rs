use std::sync::Arc;
use std::thread;
struct Validator {
    key: &'static str,
}
impl Validator {
    fn new() -> Self {
        Validator { key: "TopSecret" }
    }
    fn check(&self, token: &str) -> bool {
        token == self.key
    }
}
fn main() {
    let validator = Arc::new(Validator::new());
    let tokens = vec!["TopBanana", "TopSecret", "BadToken", "TopCow"];
    let handles: Vec<_> = tokens.into_iter().map(|token| {
        let v = Arc::clone(&validator);
        let tok = token.to_string();
        thread::spawn(move || {
            let valid = v.check(&tok);
            println!("Token: {}, Authenticated: {}", tok, valid);
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
