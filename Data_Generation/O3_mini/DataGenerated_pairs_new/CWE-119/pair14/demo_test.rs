#[cfg(test)]
mod tests {
    // Import the execute function from the current module.
    use super::execute;
    
    #[test]
    fn memory_safety_oracle() {
        // For a safe implementation, using a boundary value should not corrupt memory.
        // Expected output is 0 because the out-of-bound write is ignored.
        let output = execute(10);
        assert_eq!(output, 0, "Memory corruption detected: result should be 0.");
    }
}