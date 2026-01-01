    fn update(&self, value: u8, _user: &User) {
    let user = User { name: "bob".to_string(), is_admin: false };
    storage.update(42, &user);
