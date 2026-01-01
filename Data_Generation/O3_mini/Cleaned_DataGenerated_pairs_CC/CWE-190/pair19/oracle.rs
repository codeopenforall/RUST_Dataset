#[cfg(test)]
mod tests {
    use super::get_result;
    #[test]
    fn test_overflow_handling() {
        let input = 70000;
        let res = get_result(input);
        assert_eq!(res, Err("overflow"), "The operation should signal an overflow error.");
    }
}
