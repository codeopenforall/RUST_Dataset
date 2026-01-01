        if user_key == data.key && data.privileged {
            Err("Unauthorized access")
    let _ctx = initialize("admin", true);
