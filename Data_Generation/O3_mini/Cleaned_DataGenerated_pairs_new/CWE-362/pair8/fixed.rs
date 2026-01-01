use std::{sync::atomic::{AtomicBool, AtomicU32, Ordering}, sync::Arc, thread, time::Duration};
struct Resource {
    flag: AtomicBool,
    init_count: AtomicU32,
}
impl Resource {
    fn new() -> Self {
        Resource {
            flag: AtomicBool::new(false),
            init_count: AtomicU32::new(0),
        }
    }
    fn process(&self) {
        if !self.flag.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(10));
            if self.flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.init_count.fetch_add(1, Ordering::AcqRel);
            }
        }
    }
    fn get_count(&self) -> u32 {
        self.init_count.load(Ordering::Relaxed)
    }
}
fn main() {
    let res = Arc::new(Resource::new());
    let mut handles = Vec::new();
    for _ in 0..10 {
        let resource = Arc::clone(&res);
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                resource.process();
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Initialization count: {}", res.get_count());
}
