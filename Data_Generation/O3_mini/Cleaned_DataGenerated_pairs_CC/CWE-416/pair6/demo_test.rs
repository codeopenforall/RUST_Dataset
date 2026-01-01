#[cfg(test)]
mod tests {
    // Import the compute function from the parent module.
    use super::*;

    #[test]
    fn test_oracle() {
        // In the fixed version, compute() must return 42.
        // In the vulnerable version, due to the use-after-free, compute() returns the reallocated value 99.
        let res = compute();
        assert_eq!(res, 42, "Oracle: Expected the computed value to be 42");
    }
}