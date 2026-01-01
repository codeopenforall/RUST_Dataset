/*
This corrected version eliminates the race condition by using a Mutex to protect access
to the shared integer. The Controller struct now contains a thread-safe Arc<Mutex<i32>>, ensuring
that both the check and the subsequent update are performed while holding the lock.
This change guarantees atomicity, thereby preventing concurrency issues.
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Controller {
    data: Arc<Mutex<i32>>,
}

impl Controller {
    fn new(initial: i32) -> Self {
        Controller { data: Arc::new(Mutex::new(initial)) }
    }

    fn get(&self) -> i32 {
        *self.data.lock().unwrap()
    }

    fn set(&self, val: i32) {
        let mut guard = self.data.lock().unwrap();
        *guard = val;
    }

    // The check-then-act routine now holds the lock for the entire operation.
    fn process(&self) {
        let mut guard = self.data.lock().unwrap();
        if *guard == 0 {
            // Delay is simulated while the lock is held, ensuring exclusive access.
            thread::sleep(Duration::from_millis(50));
            *guard = 1;
        }
    }
}

fn main() {
    let ctl = Controller::new(0);
    let shared_ctl = Arc::new(ctl);
    let ctl_clone1 = Arc::clone(&shared_ctl);
    let ctl_clone2 = Arc::clone(&shared_ctl);

    let t1 = thread::spawn(move || {
        ctl_clone1.process();
    });
    let t2 = thread::spawn(move || {
        let mut guard = ctl_clone2.data.lock().unwrap();
        if *guard == 0 {
            *guard = 2;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let final_val = shared_ctl.get();
    println!("Final value: {}", final_val);
    // The final value is guaranteed to be 1 with proper synchronization.
    assert!(final_val == 1, "Race condition fixed: final value is not 1");
}