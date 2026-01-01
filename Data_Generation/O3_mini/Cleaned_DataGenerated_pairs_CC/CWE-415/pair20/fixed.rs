use std::sync::Arc;
use std::thread;
pub struct Resource {
    data: Arc<i32>,
}
impl Resource {
    pub fn new(val: i32) -> Self {
        Resource { data: Arc::new(val) }
    }
}
impl Clone for Resource {
    fn clone(&self) -> Self {
        Resource { data: Arc::clone(&self.data) }
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
