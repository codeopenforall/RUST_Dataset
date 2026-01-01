struct Memory {
    ptr: *mut i32,
}
impl Memory {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        let raw = Box::into_raw(b);
        Memory { ptr: raw }
    }
    unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}
impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
fn run() -> i32 {
    let mem = Memory::new(256);
    let value = unsafe { mem.get() };
    drop(mem);
    value
}
fn main() {
    println!("Result: {}", run());
}
