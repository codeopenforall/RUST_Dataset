#[cfg(test)]
mod tests {
    use std::panic;
    use super::{process_data};
    #[test]
    fn oracle() {
        let vulnerable_result = panic::catch_unwind(|| {
            process_data("15").unwrap()
        });
        assert!(vulnerable_result.is_err(), "The vulnerable variant did not panic on out-of-bound input.");
        let fixed_result = process_data("15");
        assert!(fixed_result.is_err(), "The fixed variant unexpectedly accepted an out-of-bound input.");
    }
}
