#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_compute() {
        // For size = 10, the intended sum is 0+1+2+...+9 = 45.
        let instance = Core::new(10);
        let computed_sum = instance.generate();
        // Test expects 45. In the vulnerable code, off-by-one error would produce 55.
        assert_eq!(computed_sum, 45, "The computed sum should be 45 for the correct iteration.");
    }
}