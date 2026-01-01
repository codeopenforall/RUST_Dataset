#[cfg(test)]
mod tests {
    use super::*;
    fn setup_engine() -> Engine {
        let engine = Engine::new();
        {
            let mut db = engine.data.lock().unwrap();
            unsafe {
                let hash = "secret".bytes().fold(0u64, |accum, b| {
                    accum.wrapping_mul(31).wrapping_add(b as u64)
                });
                db.insert("alice".to_string(), Account { name: "alice".into(), secret: hash });
            }
        }
        engine
    }
    #[test]
    fn test_authentication_error_message() {
        let engine = setup_engine();
        let res = engine.verify("alice", "guess");
        assert!(res.is_err());
        let err_msg = res.err().unwrap();
        assert_eq!(err_msg, "Authentication failed", "The error message should be generic to prevent information leaks");
    }
}
