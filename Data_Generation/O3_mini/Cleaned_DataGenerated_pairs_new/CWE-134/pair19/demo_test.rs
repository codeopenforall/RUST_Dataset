//////////////////////////////////////
// Test oracle for format processing //
//////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the common processing function.
    use super::process;

    #[test]
    fn test_format() {
        // Use a malicious input that includes format specifiers.
        let input = "Hello %x";
        // For the secure version, the fixed "%s" formatting will print the entire input literally.
        // The process function runs in two threads and sums the printed character counts.
        // Hence, the expected printed characters count equals twice the length of the input.
        let expected = input.len() as i32 * 2;
        let result = process(input);
        assert_eq!(result, expected, "Printed character count does not match the safe behavior expected.");
    }
}