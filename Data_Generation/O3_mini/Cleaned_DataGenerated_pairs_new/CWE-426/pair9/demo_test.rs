#[cfg(test)]
mod tests {
    use super::execute_task;

    #[test]
    fn test_loading() {
         // For the fixed version, the absolute path yields a secure loading check and execute_task() returns true.
         // For the vulnerable version, a relative path is used and execute_task() returns false.
         assert!(execute_task(), "Insecure loading detected: command path is relative");
    }
}