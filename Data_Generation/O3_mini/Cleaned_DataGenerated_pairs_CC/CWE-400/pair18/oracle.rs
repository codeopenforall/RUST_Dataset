#[cfg(test)]
mod tests {
    use super::simulate;
    #[test]
    fn check_resource_limits() {
        let result = simulate();
        assert!(result, "Resource consumption exceeded safe limits");
    }
}
