use std::ptr;
struct Data {
    ptr: *mut u8,
    size: usize,
}
impl Data {
    fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            return Err("Zero size");
        }
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); }
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec);
        Ok(Data { ptr, size })
    }
    fn write(&self, index: usize, val: u8) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }
        unsafe {
            *self.ptr.add(index) = val;
        }
        Ok(())
    }
}
impl Default for Data {
    fn default() -> Self {
        Data { ptr: ptr::null_mut(), size: 0 }
    }
}
fn run(input: usize) -> Result<u8, &'static str> {
    let d = Data::new(input)?;
    d.write(0, 42)?;
    Ok(42)
}
fn main() {
    match run(10) {
        Ok(val) => println!("Operation result: {}", val),
        Err(e) => println!("Operation error: {}", e),
    }
}
