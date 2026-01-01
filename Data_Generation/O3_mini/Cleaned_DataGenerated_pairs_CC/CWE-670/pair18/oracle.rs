#[cfg(test)]
mod tests {
    use super::Executor;
    #[test]
    fn contract_executor() {
        let mut exec = Executor { special: true, value: 0 };
        let result = exec.process_input(4);
        assert_eq!(
            result, 54,
            "Contract violation: expected result 54, got {}",
            result
        );
    }
}
