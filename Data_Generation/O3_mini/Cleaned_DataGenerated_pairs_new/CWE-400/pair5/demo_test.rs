#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test function should fail for the vulnerable implementation (which has no limit)
    // by accumulating more tasks than expected, and pass for the fixed implementation 
    // where tasks are capped at 10,000.
    #[test]
    fn resource_limit_oracle() {
        // Use a closure to run the test against either implementation.
        // For vulnerable code, WorkQueue::new() returns an Arc<WorkQueue>
        // and add_task() does not enforce any limit.
        // For fixed code, the cap is applied so that only 10,000 tasks are stored.
        let queue = WorkQueue::new();
        let mut handles = Vec::new();

        // Spawn 100 threads, each submitting 150 tasks => 15,000 attempted tasks.
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
        // For the fixed version, the count must be capped to 10,000.
        // The vulnerable version would accumulate all 15,000 tasks.
        assert!(
            count == 10000,
            "Test failed: Expected task count of 10000, got {}",
            count
        );
    }
}