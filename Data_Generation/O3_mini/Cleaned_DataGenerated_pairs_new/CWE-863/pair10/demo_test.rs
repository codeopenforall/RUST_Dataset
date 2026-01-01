#[cfg(test)]
mod tests {
    use super::*;

    // Test oracle: When given an input exceeding the allowed limit,
    // the corrected version spawns only up to MAX_TASKS threads.
    // For the vulnerable version, this test would fail as it spawns all threads.
    #[test]
    fn test_resource_limit() {
        let mut manager = ResourceManager::new();
        // Use an input value that is above the defined cap.
        let spawned = manager.execute(150);
        // For the fixed version, we expect the spawn count to be capped.
        assert_eq!(spawned, 100, "The number of spawned tasks should be capped at 100.");
    }
}