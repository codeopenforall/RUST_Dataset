use std::ptr;
struct Processor {
    value: i32,
}
impl Processor {
    fn update(&mut self, new_val: i32) {
        let ptr: *mut i32 = &mut self.value;
        let res = unsafe { change(ptr, new_val) };
        if res.is_ok() {
        } else {
            self.value = new_val;
        }
    }
}
unsafe fn change(ptr: *mut i32, new_val: i32) -> Result<(), i32> {
    if new_val == 42 {
        Err(new_val)
    } else {
        *ptr = new_val;
        Ok(())
    }
}
fn main() {
    let mut p = Processor { value: 10 };
    p.update(42);
    println!("Value: {}", p.value);
}
