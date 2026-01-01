use std::thread;
use std::ptr;
struct Object {
    data: i32,
}
impl Object {
    fn new(val: i32) -> Self {
        Object { data: val }
    }
}
struct Handler {
    ptr: *const Object,
}
impl Handler {
    fn new(raw: *const Object) -> Self {
        Handler { ptr: raw }
    }
    fn fetch(&self) -> i32 {
        unsafe { (*self.ptr).data }
    }
}
pub fn run() -> i32 {
    let obj = Box::new(Object::new(123));
    let raw_ptr = Box::into_raw(obj);
    let handler = Handler::new(raw_ptr);
    unsafe {
        Box::from_raw(raw_ptr);
    }
    handler.fetch()
}
fn main() {
    let handle = thread::spawn(|| {
        let val = run();
        println!("Final value: {}", val);
    });
    handle.join().unwrap();
}
