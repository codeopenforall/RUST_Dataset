#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn deep_nesting_oracle() {
        let tree = construct(1500);
        let result = panic::catch_unwind(|| {
            collect(&tree)
        });
        if result.is_err() {
            assert!(true, "Vulnerable version panicked as expected.");
        } else {
            let res = result.unwrap();
            match res {
                Ok(val) => panic!("Expected error due to deep recursion, but got value: {}", val),
                Err(msg) => assert_eq!(msg, "Recursion too deep", "Unexpected error message"),
            }
        }
    }
}
