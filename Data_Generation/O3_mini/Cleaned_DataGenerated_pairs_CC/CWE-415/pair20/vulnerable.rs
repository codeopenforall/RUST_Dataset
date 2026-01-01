use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
pub struct Resource {
    data: *mut i32,
    flag: *mut AtomicBool,
}
impl Resource {
    pub fn new(val: i32) -> Self {
        let data = Box::into_raw(Box::new(val));
        let flag = Box::into_raw(Box::new(AtomicBool::new(false)));
        Resource { data, flag }
    }
}
impl Clone for Resource {
    fn clone(&self) -> Self {
        Resource {
            data: self.data,
            flag: self.flag,
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if (*self.flag).compare_and_swap(false, true, Ordering::SeqCst) == false {
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.flag);
            } else {
                panic!("Double free detected");
            }
        }
    }
}
fn main() {
    let resource = Resource::new(100);
    let handle = thread::spawn({
        let clone_resource = resource.clone();
        move || {
            let _ = clone_resource;
        }
    });
    handle.join().unwrap();
}
