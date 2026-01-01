use std::sync::Arc;
use std::thread;
struct Object {
    data: i32,
}
impl Object {
    fn new(val: i32) -> Self {
        Object { data: val }
    }
}
struct Handler {
    handle: Arc<Object>,
}
impl Handler {
    fn new(handle: Arc<Object>) -> Self {
        Handler { handle }
    }
    fn fetch(&self) -> i32 {
        self.handle.data
    }
}
pub fn run() -> i32 {
    let obj = Arc::new(Object::new(123));
    let handler = Handler::new(Arc::clone(&obj));
    drop(obj);
    handler.fetch()
}
fn main() {
    let handle = thread::spawn(|| {
        let val = run();
        println!("Final value: {}", val);
    });
    handle.join().unwrap();
}
