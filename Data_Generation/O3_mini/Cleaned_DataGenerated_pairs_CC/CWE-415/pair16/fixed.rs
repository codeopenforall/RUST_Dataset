use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_DROP: AtomicUsize = AtomicUsize::new(0);
struct Resource {
    data: i32,
}
impl Resource {
    fn new(val: i32) -> Self {
        Resource { data: val }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        GLOBAL_DROP.fetch_add(1, Ordering::SeqCst);
    }
}
fn process_resource() {
    unsafe {
        let boxed = Box::new(Resource::new(100));
        let raw = Box::into_raw(boxed);
        {
            let _holder = Box::from_raw(raw);
        }
    }
}
fn main() {
    process_resource();
    println!("Global drop count: {}", GLOBAL_DROP.load(Ordering::SeqCst));
}
