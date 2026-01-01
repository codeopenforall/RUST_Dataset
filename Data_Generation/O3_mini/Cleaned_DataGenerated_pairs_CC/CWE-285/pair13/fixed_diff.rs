    fn update(&self, value: u8, user: &User) -> Result<(), &'static str> {
        if !user.is_admin {
            return Err("not authorized");
        }
        Ok(())
    let user = User { name: "alice".to_string(), is_admin: true };
    storage.update(99, &user).expect("update failed");
