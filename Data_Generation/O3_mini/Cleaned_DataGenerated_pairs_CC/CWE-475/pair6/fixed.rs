use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
}
trait MemoryOps {
    fn duplicate(&self, size: usize) -> Result<(), &'static str>;
}
struct DataHolder {
    store: Arc<Mutex<Vec<u8>>>,
}
impl DataHolder {
    fn new(len: usize) -> Self {
        DataHolder { store: Arc::new(Mutex::new(vec![1u8; len])) }
    }
    fn get_ptr_and_len(&self) -> (*mut u8, usize) {
        let guard = self.store.lock().unwrap();
        (guard.as_ptr() as *mut u8, guard.len())
    }
}
impl MemoryOps for DataHolder {
    fn duplicate(&self, size: usize) -> Result<(), &'static str> {
        let (ptr, actual_len) = self.get_ptr_and_len();
        if size > actual_len {
            return Err("Requested copy size exceeds allocated buffer length");
        }
        unsafe {
            if ptr.is_null() {
                return Err("Null pointer encountered");
            }
            memcpy(ptr, ptr, size);
        }
        Ok(())
    }
}
fn main() {
    let holder = DataHolder::new(10);
    let shared = Arc::new(holder);
    let workers: Vec<_> = (0..2).map(|_| {
        let handle = Arc::clone(&shared);
        thread::spawn(move || {
            let _ = handle.duplicate(10);
        })
    }).collect();
    for worker in workers {
        let _ = worker.join();
    }
}
