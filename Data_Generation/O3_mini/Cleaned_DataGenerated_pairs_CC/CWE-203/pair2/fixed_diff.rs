unsafe fn constant_time_compare(stored: &str, provided: &str) -> bool {
    let n = stored.len().max(provided.len());
    let mut result: u8 = 0;
    for i in 0..n {
        let sc = if i < stored.len() { *stored.as_ptr().add(i) } else { 0 };
        let pc = if i < provided.len() { *provided.as_ptr().add(i) } else { 0 };
        result |= sc ^ pc;
    (result == 0) && (stored.len() == provided.len())
        let stored = self.accounts.get(username).map(|s| s.as_str()).unwrap_or("");
        if !stored.is_empty() && unsafe { constant_time_compare(stored, password) } {
            Ok(())
            Err("Invalid credentials")
    let auth = AccessControl::new();
    let auth_arc = Arc::new(auth);
    let cloned = Arc::clone(&auth_arc);
