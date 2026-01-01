use std::ptr;
struct Container {
    inner: Vec<u8>,
}
impl Container {
    fn new(size: usize) -> Self {
        Container { inner: vec![0; size] }
    }
    fn modify(&mut self, start: isize, count: usize, val: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(start);
            for i in 0..count {
                *ptr.offset(i as isize) = val;
            }
        }
        Ok(())
    }
}
fn main() {
    let mut obj = Container::new(10);
    match obj.modify(0, 12, 42) {
        Ok(_) => println!("Modification completed."),
        Err(e) => println!("Error: {}", e),
    }
}
