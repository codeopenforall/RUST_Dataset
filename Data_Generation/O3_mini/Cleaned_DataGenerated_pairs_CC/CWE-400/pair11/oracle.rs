#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use super::Service;
    #[test]
    fn resource_capping_test() {
        let service = Arc::new(Mutex::new(Service::new()));
        let svc = Arc::clone(&service);
        let worker = thread::spawn(move || {
            for _ in 0..20 {
                if let Ok(mut s) = svc.lock() {
                    s.compute(1);
                }
                thread::sleep(Duration::from_millis(1));
            }
        });
        worker.join().unwrap();
        let s = service.lock().unwrap();
        assert_eq!(s.data.len(), 10, "Resource consumption capping failed");
    }
}
