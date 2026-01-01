use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
const SAFE_LIMIT: usize = 500;
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);
        if num > SAFE_LIMIT {
            return Err(format!(
                "Requested number {} exceeds safe limit {}",
                num, SAFE_LIMIT
            ));
        }
                TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            handle.join().unwrap();
    match service.execute_request(1000) {
        Ok(_) => println!(
            "Total tasks processed: {}",
            TASK_COUNT.load(Ordering::SeqCst)
        ),
        Err(e) => eprintln!("Error: {}", e),
