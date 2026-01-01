use std::fmt;
impl fmt::Debug for Cred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cred")
         .field("user", &self.user)
         .field("secret", &"******")
         .finish()
    }
}
    format!("{:?}", *guard)
