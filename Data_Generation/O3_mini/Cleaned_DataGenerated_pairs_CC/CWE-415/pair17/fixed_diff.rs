use std::sync::{Arc, Barrier, Mutex};
struct Resource {
    inner: Arc<Mutex<Option<*mut i32>>>,
impl Resource {
        Resource {
            inner: Arc::new(Mutex::new(Some(Box::into_raw(boxed)))),
        let guard = self.inner.lock().unwrap();
        if let Some(raw) = *guard {
            unsafe { *raw }
        } else {
            panic!("Attempted to access resource after deallocation");
        }
impl Drop for Resource {
        let mut guard = self.inner.lock().unwrap();
        if let Some(raw) = guard.take() {
            unsafe {
                Box::from_raw(raw);
            }
    let resource = Resource::new(500);
    let shared = Arc::clone(&resource.inner);
        let guard = shared.lock().unwrap();
        if let Some(raw) = *guard {
            let val = unsafe { *raw };
            println!("Thread sees value: {}", val);
    println!("Value: {}", resource.get());
