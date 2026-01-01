////////////////////////////////////////////////////////////
// Test Oracle:
// This test checks the behavior of the Modifier::modify method.
// A valid in-bound modification should succeed, while an out-of-bound
// modification should return an error. In the vulnerable version, the
// out-of-bound call improperly succeeds (leading to memory corruption),
// whereas in the fixed version, the call fails as expected.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::{DataPool, Modifier};

    #[test]
    fn test_bounds_check() {
        // Create a DataPool of size 10.
        let mut dp = DataPool::new(10);

        // In-bound modification should work.
        assert!(dp.modify(5, 100).is_ok(), "In-bound modification must succeed");

        // Out-of-bound modification should be rejected.
        let result = dp.modify(11, 200);
        assert!(result.is_err(), "Out-of-bound modification should return an error");
    }
}