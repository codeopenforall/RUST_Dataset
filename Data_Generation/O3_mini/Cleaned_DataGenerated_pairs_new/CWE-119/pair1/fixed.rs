use std::ptr;
struct Container {
    inner: Vec<u8>,
}
impl Container {
    fn new(size: usize) -> Self {
        Container { inner: vec![0; size] }
    }
    fn modify(&mut self, start: isize, count: usize, val: u8) -> Result<(), &'static str> {
        if start < 0 {
            return Err("start index is negative");
        }
        let start = start as usize;
        if start.checked_add(count).filter(|&end| end <= self.inner.len()).is_none() {
            return Err("Attempt to write beyond buffer boundaries");
        }
        unsafe {
            let ptr = self.inner.as_mut_ptr().add(start);
            for i in 0..count {
                *ptr.add(i) = val;
            }
        }
        Ok(())
    }
}
fn main() {
    let mut obj = Container::new(10);
    match obj.modify(0, 10, 42) {
        Ok(_) => println!("Modification completed safely."),
        Err(e) => println!("Error: {}", e),
    }
}
