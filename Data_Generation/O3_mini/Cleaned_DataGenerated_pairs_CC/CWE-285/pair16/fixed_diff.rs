    pub fn get_secret(&self, user: &User) -> Result<String, Error> {
        if !user.is_admin {
            return Err(Error::Unauthorized);
        }
        username: "admin".to_string(),
        is_admin: true,
