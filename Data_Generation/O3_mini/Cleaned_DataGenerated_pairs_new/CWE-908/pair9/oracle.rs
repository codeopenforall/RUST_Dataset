#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_initialization() {
        let resource = create_resource();
        assert_eq!(resource.number, 100, "Number field should be 100");
        assert_eq!(
            resource.description,
            "Initialized",
            "Description field is not properly initialized"
        );
    }
}
