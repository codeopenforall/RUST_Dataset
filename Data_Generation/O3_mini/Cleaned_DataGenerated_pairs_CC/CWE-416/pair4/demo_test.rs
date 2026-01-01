/*
   The following test oracle verifies that the computation result is 42. For the vulnerable version,
   due to the use-after-free, the result is likely to be incorrect (or could cause a panic), causing the test
   to fail. In contrast, the corrected version returns the expected value, making the test pass.
*/

#[cfg(test)]
mod tests {
    // Change the import path as needed to point to the module under test.
    use super::compute;

    #[test]
    fn test_compute_result() {
        let res = compute();
        // Assert that the computed value is as expected.
        assert_eq!(res, 42, "Test failed: expected 42, got {}", res);
    }
}