//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction_behavior() {
        // Create a new accumulator starting at 0.
        let mut accumulator = Accumulator::new(0);
        // Attempt to subtract 1.
        accumulator.update(1);
        // For the fixed code, subtracting from 0 should yield 0.
        // In the vulnerable code, the result would underflow (e.g., 4294967295 for u32).
        assert_eq!(accumulator.value(), 0, "Underflow detected: the value is not safe");
    }
}