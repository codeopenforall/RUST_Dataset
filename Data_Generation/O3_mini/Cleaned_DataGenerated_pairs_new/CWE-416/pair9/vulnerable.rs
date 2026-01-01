pub trait Worker {
    fn evaluate(&self) -> u32;
}
pub struct Processor {
    ptr: *mut u32,
}
impl Processor {
    pub fn new(val: u32) -> Self {
        let b = Box::new(val);
        Self {
            ptr: Box::into_raw(b),
        }
    }
    pub unsafe fn compute(&self) -> u32 {
        *self.ptr
    }
}
impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
pub fn execute() -> u32 {
    let result: u32;
    let local_ptr: *mut u32;
    {
        let job = Processor::new(42);
        local_ptr = job.ptr;
    } 
    unsafe {
        result = *local_ptr;
    }
    result
}
fn main() {
    let res = execute();
    println!("Result is: {}", res);
}
