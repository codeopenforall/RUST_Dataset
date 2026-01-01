unsafe fn compare_passwords(stored: &str, provided: &str) -> bool {
    if stored.len() != provided.len() {
        return false;
    let stored_ptr = stored.as_ptr();
    let provided_ptr = provided.as_ptr();
    for i in 0..stored.len() {
        if *stored_ptr.add(i) != *provided_ptr.add(i) {
            return false;
        }
    }
    true
        if let Some(stored) = self.accounts.get(username) {
            if unsafe { compare_passwords(stored, password) } {
                Ok(())
            } else {
                Err("Incorrect password for existing user")
            }
            Err("User not found")
    let system = AccessControl::new();
    let system_arc = Arc::new(system);
    let cloned = Arc::clone(&system_arc);
