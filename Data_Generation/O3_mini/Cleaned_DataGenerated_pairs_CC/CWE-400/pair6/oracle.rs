#[cfg(test)]
mod tests {
    use std::panic;
    use super::run_simulation;
    #[test]
    fn integration_test() {
         let result = panic::catch_unwind(|| {
             run_simulation(1000)
         });
         match result {
             Ok(Ok(count)) => {
                 assert_eq!(count, 4000, "Incorrect number of processed messages.");
             }
             Ok(Err(e)) => {
                 panic!("Function returned an error: {}", e);
             }
             Err(_) => {
                 panic!("Function panicked, indicating uncontrolled resource consumption vulnerability.");
             }
         }
    }
}
