#[cfg(test)]
mod tests {
    use super::run_simulation;

    #[test]
    fn test_deallocation() {
        // When run_simulation is executed, it should deallocate the memory exactly once.
        let drop_result = run_simulation();
        assert_eq!(
            drop_result, 
            1, 
            "Expected exactly one drop (proper deallocation). A different count indicates a double free vulnerability."
        );
    }
}