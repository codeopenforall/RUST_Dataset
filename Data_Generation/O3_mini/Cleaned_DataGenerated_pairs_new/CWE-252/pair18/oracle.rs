#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn verify_final_state() {
        let final_value = run();
        assert_eq!(final_value, 100, "Final value is unsafe due to unchecked error handling");
    }
}
