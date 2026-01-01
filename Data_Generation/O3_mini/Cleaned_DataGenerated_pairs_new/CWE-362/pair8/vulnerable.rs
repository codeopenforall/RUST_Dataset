use std::{cell::UnsafeCell, sync::Arc, thread, time::Duration};
struct Resource {
    flag: UnsafeCell<bool>,
    init_count: UnsafeCell<u32>,
}
unsafe impl Sync for Resource {}
impl Resource {
    fn new() -> Self {
        Resource {
            flag: UnsafeCell::new(false),
            init_count: UnsafeCell::new(0),
        }
    }
    fn process(&self) {
        unsafe {
            if !*self.flag.get() {
                thread::sleep(Duration::from_millis(10));
                *self.init_count.get() = *self.init_count.get() + 1;
                *self.flag.get() = true;
            }
        }
    }
    fn get_count(&self) -> u32 {
        unsafe { *self.init_count.get() }
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
