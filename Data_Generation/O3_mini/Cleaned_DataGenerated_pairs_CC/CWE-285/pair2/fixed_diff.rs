    fn retrieve(&self, user: &User) -> Result<i32, &'static str> {
        if user.role == "admin" && self.admin_flag {
            Ok(self.data)
        } else {
            Err("Unauthorized access")
        }
    guard.retrieve(user)
    let user = User { id: 1, role: "admin".to_string() };
