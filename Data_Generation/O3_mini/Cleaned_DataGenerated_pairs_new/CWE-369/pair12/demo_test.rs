#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn division_by_zero_oracle() {
        let calc = Calculator;
        // Using the unified API wrapper safe_compute, which in the fixed version returns Err("Division by zero error")
        // whereas in the vulnerable version, a panic is caught and translated to Err("Panic occurred").
        let result = calc.safe_compute(10, 0);
        match result {
            Ok(val) => panic!("Expected an error when dividing by zero, but got Ok({})", val),
            Err(ref msg) if msg == "Division by zero error" => (), // Correct behavior in the fixed version.
            Err(ref msg) => panic!("Unexpected error message: {}", msg),
        }
    }
}