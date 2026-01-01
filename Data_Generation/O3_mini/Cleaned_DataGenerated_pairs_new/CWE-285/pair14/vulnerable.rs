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
    fn modify(&self, _caller: &str, new_val: i32) -> Result<(), &'static str> {
        let data_ptr = {
            let guard = self.inner.lock().unwrap();
            let ptr = &*guard as *const Data as *mut Data;
            ptr
        };
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
        let _ = controller.modify("intruder", 100);
    });
    handle.join().unwrap();
    println!("Final secret value: {}", ctrl.read());
}
