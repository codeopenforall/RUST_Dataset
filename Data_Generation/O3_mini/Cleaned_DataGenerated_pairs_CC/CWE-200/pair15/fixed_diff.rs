}
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("secret", &"[REDACTED]")
            .finish()
    }
        format!("Error encountered for credentials: {:?}", *creds)
                let _ = creds_arc.lock().unwrap().secret.clone();
