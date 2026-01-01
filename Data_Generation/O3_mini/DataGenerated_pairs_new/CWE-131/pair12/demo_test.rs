//////////////////////////////////////////////////////////////
// Test Oracle: 
// The following testing module is provided as a unit test. It creates a buffer from the input string,
// then checks that the resulting string matches the expected value. For the fixed code this test passes,
// but in the vulnerable code the miscalculated buffer size can lead to undefined behavior or incorrect values,
// thus causing the test to fail.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn buffer_integrity() {
        let input = "Hello, world!";
        unsafe {
            let buf = Data::create(input);
            let result = buf.as_text();
            assert_eq!(result, input, "Buffer content does not match expected input");
        }
    }
}