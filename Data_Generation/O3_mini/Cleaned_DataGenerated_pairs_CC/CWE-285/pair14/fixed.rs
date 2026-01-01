use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    owner: String,
    secret: i32,
}
impl Data {
    fn new(owner: &str, secret: i32) -> Self {
        Data {
            owner: owner.to_string(),
            secret,
        }
    }
}
struct Controller {
    inner: Arc<Mutex<Data>>,
}
impl Controller {
    fn new(owner: &str, secret: i32) -> Self {
        Controller {
            inner: Arc::new(Mutex::new(Data::new(owner, secret))),
        }
    }
    fn modify(&self, caller: &str, new_val: i32) -> Result<(), &'static str> {
        let mut guard = self.inner.lock().unwrap();
        if caller != guard.owner {
            return Err("Unauthorized access");
        }
        let data_ptr = &mut *guard as *mut Data;
        unsafe {
            (*data_ptr).secret = new_val;
        }
        Ok(())
    }
    fn read(&self) -> i32 {
        let guard = self.inner.lock().unwrap();
        guard.secret
    }
}
fn main() {
    let ctrl = Controller::new("admin", 42);
    let ctrl_clone = ctrl.inner.clone();
    let handle = thread::spawn(move || {
        let controller = Controller { inner: ctrl_clone };
        if let Err(e) = controller.modify("intruder", 100) {
            println!("Access denied: {}", e);
        }
    });
    handle.join().unwrap();
    println!("Final secret value: {}", ctrl.read());
}
