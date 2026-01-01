#[cfg(test)]
mod tests {
    use super::evaluate;
    use std::panic;
    #[test]
    fn test_deep_input() {
        let deep_input = "(".repeat(5000);
        let result = panic::catch_unwind(|| {
            evaluate(&deep_input)
        });
        match result {
            Ok(eval_result) => {
                match eval_result {
                    Err(err_msg) => assert_eq!(err_msg, "Maximum recursion depth exceeded", "Fixed version should return the recursion depth error"),
                    Ok(val) => panic!("Expected an error due to deep recursion, but got a value: {}", val),
                }
            },
            Err(_) => panic!("The evaluation panicked, indicating uncontrolled recursion (vulnerable behavior)")
        }
    }
}
