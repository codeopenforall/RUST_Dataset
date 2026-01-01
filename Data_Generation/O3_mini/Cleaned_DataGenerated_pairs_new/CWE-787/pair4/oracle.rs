#[cfg(test)]
mod tests {
    use std::sync::{Mutex, Arc};
    use std::thread;
    use super::{Worker, Summable};
    #[test]
    fn oracle_test() {
        let worker = Worker::new(5);
        let worker = Arc::new(Mutex::new(worker));
        let worker_clone = Arc::clone(&worker);
        let handle = thread::spawn(move || {
            let mut w = worker_clone.lock().unwrap();
            w.run();
        });
        handle.join().unwrap();
        let total = worker.lock().unwrap().total();
        assert_eq!(total, 100, "Oracle check failed: total is not equal to 100");
    }
}
