use std::mem;
#[derive(Debug)]
struct Data {
    value: i32,
}
trait Action {
    fn execute(&self) -> i32;
}
struct Handler {
    ptr: *const Data,
}
impl Action for Handler {
    fn execute(&self) -> i32 {
        unsafe {
            (*self.ptr).value  
        }
    }
}
pub fn compute() -> i32 {
    let data_box = Box::new(Data { value: 42 });
    let raw = Box::into_raw(data_box);
    let handler = Handler { ptr: raw };
    unsafe {
        let _freed = Box::from_raw(raw);
    }
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        mem::forget(dummy);
    }
    let result = handler.execute();
    result
}
fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}
