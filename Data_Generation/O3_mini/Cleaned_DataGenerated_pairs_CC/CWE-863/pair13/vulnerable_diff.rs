    unsafe {
        if std::ptr::eq(user.as_ptr(), "admin".as_ptr()) {
            if amount > MAX_LIMIT {
                return Err("Authorized request exceeds the permitted limit".to_string());
            }
        } else {
        }
        ("admin", 512 * 1024),      
