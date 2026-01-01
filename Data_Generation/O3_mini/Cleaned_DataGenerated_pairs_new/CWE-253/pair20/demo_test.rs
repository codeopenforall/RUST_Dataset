fn oracle() {
    // Test Setup:
    // For a negative input (-1) the corrected code should not update the vector.
    // In contrast, the vulnerable code erroneously updates the vector.
    let engine = Engine::new();
    engine.execute(-1);
    let data = engine.get_data();
    // The invariant: With proper error handling the vector should remain empty.
    assert!(data.is_empty(), "Test failed: Data was updated with a negative value");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_input() {
        oracle();
    }
}