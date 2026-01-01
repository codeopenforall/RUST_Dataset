use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Container {
    raw: *mut i32,
}
impl Container {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Self { raw: Box::into_raw(boxed) }
    }
    unsafe fn free_resource(&mut self) {
        if !self.raw.is_null() {
            drop(Box::from_raw(self.raw));
            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
}
fn process() -> Result<(), &'static str> {
    let mut obj = Container::new(50);
    unsafe {
        obj.free_resource(); 
        obj.free_resource(); 
    }
    if GLOBAL_COUNT.load(Ordering::SeqCst) == 1 {
        Ok(())
    } else {
        Err("Double free occurred")
    }
}
fn main() {
    let result = process();
    println!("Deallocation count: {}", GLOBAL_COUNT.load(Ordering::SeqCst));
    match result {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
