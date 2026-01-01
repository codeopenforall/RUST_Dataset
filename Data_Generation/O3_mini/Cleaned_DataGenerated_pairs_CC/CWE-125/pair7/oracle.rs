#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_boundary() {
        let res = execute();
        assert_eq!(res, 3, "Expected result 3 from safe access but got a different value");
    }
}
