}
impl fmt::Debug for Creds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "Creds {{ user: {:?}, pass: \"REDACTED\" }}", self.user)
    }
        let safe_info = format!("Processed request for user: {:?}", creds.user);
        return Err(safe_info);
        let res = service.process();
        if res.is_err() {
            println!("Operation encountered an error: {}", res.unwrap_err());
        }
    println!("Application safely exiting");
