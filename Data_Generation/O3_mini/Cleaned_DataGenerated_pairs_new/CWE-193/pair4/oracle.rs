#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_compute() {
        let instance = Core::new(10);
        let computed_sum = instance.generate();
        assert_eq!(computed_sum, 45, "The computed sum should be 45 for the correct iteration.");
    }
}
