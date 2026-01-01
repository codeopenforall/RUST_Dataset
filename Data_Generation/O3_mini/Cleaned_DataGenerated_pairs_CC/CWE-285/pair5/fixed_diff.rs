    if !user.is_admin {
        return Err("Unauthorized access");
    }
    let user = User { name: String::from("admin_user"), is_admin: true };
