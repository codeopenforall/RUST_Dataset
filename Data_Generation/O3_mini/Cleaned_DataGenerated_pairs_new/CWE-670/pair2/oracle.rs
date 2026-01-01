#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contract_test() {
        let proc = Processor::new();
        let ret = proc.run(true);
        assert_eq!(ret, 100, "Return value should be 100 when condition is true");
        assert_eq!(proc.get_state(), Mode::Active, "State should be Active when condition is true");
    }
}
