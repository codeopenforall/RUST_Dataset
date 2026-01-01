/* 
   Test Oracle:
   This test function verifies that calling the calculation function with zero as the input 
   does not cause a panic. It uses std::panic::catch_unwind to capture any unexpected panic.
   In the safe revision, invoking the calculation with a zero divisor should return None.
   In the vulnerable version the same call is expected to panic, leading this test to fail.
*/
#[cfg(test)]
mod tests {
    use super::Math;
    use std::panic;

    #[test]
    fn test_division_zero() {
        let math = Math;
        let outcome = panic::catch_unwind(|| math.calc(0));
        match outcome {
            Err(_) => panic!("Test failed: unexpected panic on zero divisor."),
            Ok(res) => {
                assert!(res.is_none(), "Test failed: Expected None for zero divisor, got {:?}", res);
            }
        }
    }
}