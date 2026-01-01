use std::boxed::Box;
struct Info {
    value: i32,
}
impl Info {
    pub fn crunch() -> i32 {
        let original = Box::new(Info { value: 42 });
        let ptr: *mut Info = Box::into_raw(original);
        unsafe {
            drop(Box::from_raw(ptr));
            let _dummy = Box::new(Info { value: 0 });
            let ret = (*ptr).value;   
            ret
        }
    }
}
fn main() {
    let res = Info::crunch();
    println!("Result: {}", res);
}
