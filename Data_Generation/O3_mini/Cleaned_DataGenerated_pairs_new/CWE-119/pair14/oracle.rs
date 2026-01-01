#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn memory_safety_oracle() {
        let output = execute(10);
        assert_eq!(output, 0, "Memory corruption detected: result should be 0.");
    }
}
