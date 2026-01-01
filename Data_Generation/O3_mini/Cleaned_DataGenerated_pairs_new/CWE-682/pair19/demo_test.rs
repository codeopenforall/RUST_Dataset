//////////////////////////////////////////////////////////////
// Test Oracle Function for Conversion Calculation            //
// This test verifies that the conversion function returns       //
// the expected value. For an input of 500 seconds, the correct    //
// conversion should yield 500,000 milliseconds.                  //
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle_test() {
        let processor = TimeProcessor::new();
        // For a 500-second input, the correct computation is 500 * 1000 = 500,000.
        let expected = 500_000;
        let actual = processor.compute(500);
        assert_eq!(actual, expected, "Conversion calculation is incorrect.");
    }
}