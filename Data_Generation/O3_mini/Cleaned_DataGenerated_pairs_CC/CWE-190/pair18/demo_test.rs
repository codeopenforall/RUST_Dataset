#[cfg(test)]
mod tests {
    // Import the compute_factorial function from the surrounding module.
    use super::*;

    #[test]
    fn overflow_detection() {
        // For input 13, the safe (fixed) implementation should detect overflow and return an error.
        // In the vulnerable version, the function returns an incorrect Ok(_) value, causing this test to fail.
        let res = compute_factorial(13);
        assert!(res.is_err(), "Expected overflow error for input 13 but got {:?}", res);
    }
}