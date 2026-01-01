#[cfg(test)]
mod tests {
    use std::panic;
    fn retrieve() -> Result<usize, &'static str> {
        let res = super::load_resource(42);
        let len = res.description.len();
        if len == 0 {
            Err("description was not properly initialized")
        } else {
            Ok(len)
        }
    }
    #[test]
    fn test_oracle() {
        let outcome = panic::catch_unwind(|| retrieve());
        match outcome {
            Ok(Ok(len)) => {
                assert_eq!(len, "Properly initialized".len());
            },
            _ => panic!("Test oracle fails: resource initialization is improper"),
        }
    }
}
