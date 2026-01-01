#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resource_allocation_oracle() {
        // Use an input value that exceeds the allowed limit.
        let input = 150;
        // Catch panics to differentiate between a safe error return and a crash.
        let result = std::panic::catch_unwind(|| process_input(input));
        match result {
            // For the safe (fixed) version, process_input should not panic but return an Err.
            Ok(inner) => {
                assert!(inner.is_err(), "Expected an error for input exceeding allowed limit");
            },
            Err(_) => {
                panic!("Function panicked instead of returning an error");
            }
        }
    }
}