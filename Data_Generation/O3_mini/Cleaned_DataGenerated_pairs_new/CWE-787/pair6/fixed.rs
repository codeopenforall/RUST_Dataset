use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Buffer { data: v }
    }
    fn assign(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        if let Some(elem) = self.data.get_mut(index) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    fn content(&self) -> &[u32] {
        &self.data
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    buf.assign(9, 999).expect("Assignment failed");
    println!("Assignment completed (fixed).");
    println!("Buffer content: {:?}", buf.content());
}
