#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn recursion_depth_oracle() {
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| process_input(&deep_input));
        match result {
            Ok(inner) => {
                match inner {
                    Ok(_) => panic!("Unexpected success on deeply nested input."),
                    Err(e) => assert_eq!(e, "maximum recursion depth exceeded", "Expected recursion limit error."),
                }
            }
            Err(_) => panic!("Function panicked due to uncontrolled recursion."),
        }
    }
}
