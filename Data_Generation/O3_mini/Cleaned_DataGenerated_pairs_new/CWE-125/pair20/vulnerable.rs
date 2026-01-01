use std::sync::Arc;
use std::thread;
struct DataContainer {
    items: Vec<u32>,
}
impl DataContainer {
    fn new(count: usize) -> Self {
        DataContainer {
            items: (0..count as u32).collect(),
        }
    }
    fn read_item(&self, index: usize) -> Option<u32> {
        unsafe { Some(*self.items.get_unchecked(index)) }
    }
}
fn main() {
    let container = Arc::new(DataContainer::new(10));
    let shared = container.clone();
    let handle = thread::spawn(move || {
        let result = shared.read_item(10);
        match result {
            Some(val) => println!("Thread read value: {}", val),
            None => println!("Thread encountered None"),
        }
    });
    let _ = handle.join();
}
