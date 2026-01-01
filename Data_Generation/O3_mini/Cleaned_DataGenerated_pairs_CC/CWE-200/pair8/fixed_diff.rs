use std::fmt;
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
         .field("username", &self.username)
         .field("password", &"REDACTED")
         .finish()
    }
}
        let safe_secret = "******"; 
        let mut log = ld.lock().unwrap();
        *log = format!("User info: {:#?}, Secret: {}", creds, safe_secret);
