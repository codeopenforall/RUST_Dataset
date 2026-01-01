//////////////////////////////////////////////////////////////
// The test oracle below exercises the compute method of the Calculator.
// When run against the flawed implementation, the result will be 200,
// causing the assertion to fail. With the corrected implementation,
// the result will be 2 and the test will pass.
//
// This test does not rely on printed output and uses explicit assertions.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn oracle_test() {
        let calc = Calculator { scale: 10 };
        // The chosen inputs: numerator=100 and denominator=5.
        // The intended correct calculation: 100/(5*10)=2.
        unsafe {
            let computed = calc.compute(100, 5);
            assert_eq!(computed, 2, "Computation error: expected 2, got {}", computed);
        }
    }
}