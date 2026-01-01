    fn retrieve(&self, _user: &User) -> i32 {
        self.data
    let raw = &*guard as *const Service;
    drop(guard);
    unsafe {
        Ok((*raw).retrieve(user))
    }
    let user = User { id: 1, role: "user".to_string() };
