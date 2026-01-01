#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // This input exceeds the typical size of a reference on 64-bit systems (usually 16 bytes)
        let input = "This is a string longer than sixteen!";
        let output = process_input(input);
        // The vulnerable version would truncate the string, causing this assertion to fail.
        assert_eq!(output, input, "Processed output does not match the input; vulnerability present.");
    }
}