use std::env;
use std::sync::Arc;
use std::thread;
pub struct Service;
impl Service {
    pub fn new() -> Self {
        Service {}
    }
    pub unsafe fn process(&self, token: Option<&str>) -> bool {
        token == Some("secret")
    }
    pub unsafe fn reserved_query(&self, token: Option<&str>) -> bool {
        token == Some("secret")
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let service = Arc::new(Service::new());
    let svc_clone = Arc::clone(&service);
    let normal_handle = thread::spawn(move || {
        unsafe { svc_clone.process(Some("secret")) }
    });
    let reserved_result = unsafe { service.reserved_query(None) };
    let normal_result = normal_handle.join().unwrap();
    println!("Normal endpoint returned: {}", normal_result);
    println!("Reserved endpoint returned: {}", reserved_result);
}
