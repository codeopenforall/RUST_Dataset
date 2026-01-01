////////////////////////////
// Test Oracle for Unittest //
////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that the resource is fully initialized.
    // The "description" field should equal "Initialized". In the vulnerable version,
    // this invariant is violated leading to test failure.
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