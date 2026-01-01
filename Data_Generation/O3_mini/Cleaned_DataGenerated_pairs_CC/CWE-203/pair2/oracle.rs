#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let auth = AccessControl::new();
        let res = auth.authenticate("alice", "wrongpass");
        assert_eq!(res, Err("Invalid credentials"));
    }
}
