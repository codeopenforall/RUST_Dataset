/*
   Test Oracle Function:
   This function drives the multiplication on potentially overflowing inputs.
   It asserts that an error is returned when the multiplication would overflow.
   For the vulnerable implementation, the assertion will fail because it returns a wrapped value.
   For the corrected version, the assertion passes.
*/
#[allow(dead_code)]
pub fn test_oracle<M: Multiply>(processor: &M) {
    let a = std::u32::MAX;
    let b = 2;
    let result = processor.multiply(a, b);
    assert!(
        result.is_err(),
        "Test failed: Overflow not detected, result was: {:?}",
        result
    );
}