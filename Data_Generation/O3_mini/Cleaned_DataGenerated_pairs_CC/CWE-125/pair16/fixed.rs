use std::env;
struct DataStore {
    buffer: Vec<u8>,
}
impl DataStore {
    fn new() -> Self {
        DataStore {
            buffer: vec![10, 20, 30, 40, 50],
        }
    }
    fn fetch(&self, position: usize) -> u8 {
        self.buffer.get(position).copied().expect("Index out-of-bounds")
    }
}
fn main() {
    let store = DataStore::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <index>", args[0]);
        return;
    }
    let pos: usize = args[1].parse().unwrap_or(0);
    let value = store.fetch(pos);
    println!("Fetched value: {}", value);
}
