#[derive(Debug)]
    password: String, 
    unsafe fn search(&self, uname: &str) -> Option<*const Record> {
        if let Some(user) = guard.get(uname) {
            Some(user as *const Record)
        } else {
            None
        }
    unsafe {
        if let Some(user_ptr) = db.search(username) {
            let user = &*user_ptr;
            if user.password == password {
                Ok(())
            } else {
                Err("Incorrect password".to_string())
            }
            Err("User not found".to_string())
