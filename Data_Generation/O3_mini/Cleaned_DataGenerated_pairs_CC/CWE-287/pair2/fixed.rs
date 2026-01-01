use std::sync::Arc;
struct Authenticator {
    secret: Arc<String>,
}
impl Authenticator {
    fn new(secret: &str) -> Self {
        Authenticator { secret: Arc::new(secret.to_string()) }
    }
    fn validate(&self, token: &str) -> Result<(), &'static str> {
        if token == self.secret.as_str() {
            Ok(())
        } else {
            Err("Invalid token")
        }
    }
}
fn main() {
    let auth = Authenticator::new("SuperSecret");
    match auth.validate("BadToken") {
        Ok(_) => println!("User Authenticated"),
        Err(e) => println!("Authentication error: {}", e),
    }
}
