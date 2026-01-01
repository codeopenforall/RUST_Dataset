#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_manager_behavior() {
        let mut tm = TaskManager::new(3);
        tm.execute(0, 100, false).expect("Execution should succeed");
        tm.execute(0, 200, true).expect("Execution should succeed");
        assert_eq!(tm.get_tasks(), &[200, 0, 0], "The TaskManager state did not match the expected invariant");
    }
}
