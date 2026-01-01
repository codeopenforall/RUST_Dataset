--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_result() {
        // Expected result is the sum of [10, 20, 30] = 60.
        // In the vulnerable version, undefined behavior might lead to an incorrect result or panic,
        // while the corrected version will always produce 60.
        let engine = Engine;
        let result = engine.execute();
        assert_eq!(result, 60, "Engine computation did not return the expected result of 60");
    }
}
--------------------------------------------------