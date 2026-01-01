#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_process_output() {
        let app = Arc::new(App::new(true));
        let app_clone = Arc::clone(&app);
        let handle = thread::spawn(move || {
            app_clone.process(0)
        });
        let result = handle.join().unwrap();
        assert_eq!(result.unwrap(), 10, "The process function did not return the expected value");
    }
}
