use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[derive(Debug)]
struct Conn {
    id: u64,
}
struct Server {
    conns: Arc<Mutex<Vec<Conn>>>,
}
pub trait ResourceManager {
    fn new(limit: usize) -> Self;
    fn run(&self);
    fn get_count(&self) -> usize;
}
impl ResourceManager for Server {
    fn new(_limit: usize) -> Self {
        Server {
            conns: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn run(&self) {
        let conns = self.conns.clone();
        thread::spawn(move || {
            let mut id_counter = 0u64;
            loop {
                unsafe {
                    let connection = Conn { id: id_counter };
                    let ptr: *mut Conn = Box::into_raw(Box::new(connection));
                    (*conns.lock().unwrap()).push(*Box::from_raw(ptr));
                    id_counter = id_counter.wrapping_add(1);
                }
            }
        });
    }
    fn get_count(&self) -> usize {
        self.conns.lock().unwrap().len()
    }
}
pub type ResourceImpl = Server;
fn main() {
    let manager = ResourceImpl::new(1000);
    manager.run();
    loop {
        thread::sleep(Duration::from_millis(500));
    }
}
