#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_interest_computation() {
        let acc = Account { balance: 100 };
        let result = acc.compute(10);
        assert_eq!(result, 110, "Interest computation did not produce the expected result");
    }
}
