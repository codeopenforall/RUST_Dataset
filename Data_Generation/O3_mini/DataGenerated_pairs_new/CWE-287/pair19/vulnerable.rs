use std::sync::Arc;
use std::thread;

struct Validator {
    key: &'static str,
}

impl Validator {
    fn new() -> Self {
        Validator { key: "TopSecret" }
    }

    // Incorrect authentication: compares only the first three bytes.
    fn check(&self, token: &str) -> bool {
        if token.len() < 3 || self.key.len() < 3 {
            return false;
        }
        unsafe {
            let token_ptr = token.as_bytes().as_ptr();
            let key_ptr = self.key.as_bytes().as_ptr();
            // Flawed: only compares the first three bytes.
            std::slice::from_raw_parts(token_ptr, 3)
                == std::slice::from_raw_parts(key_ptr, 3)
        }
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