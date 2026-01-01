#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue_limit() {
        let svc = Service::new();
        let mut error_detected = false;
        for i in 0..1100 {
            if svc.submit(format!("Task {}", i)).is_err() {
                error_detected = true;
                break;
            }
        }
        assert!(error_detected, "Expected rejection when queue limit is exceeded");
    }
}
