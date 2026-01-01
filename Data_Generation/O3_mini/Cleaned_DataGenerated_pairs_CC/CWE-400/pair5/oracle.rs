#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn resource_limit_oracle() {
        let queue = WorkQueue::new();
        let mut handles = Vec::new();
        for i in 0..100 {
            let q = Arc::clone(&queue);
            handles.push(thread::spawn(move || {
                for j in 0..150 {
                    q.add_task(i * 150 + j);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let count = queue.len();
        assert!(
            count == 10000,
            "Test failed: Expected task count of 10000, got {}",
            count
        );
    }
}
