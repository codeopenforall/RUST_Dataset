use std::sync::Arc;
use std::thread;
struct Data {
    arr: Vec<u8>,
}
impl Data {
    fn new() -> Self {
        Data { arr: vec![10, 20, 30, 40, 50] }
    }
    fn get_at(&self, pos: usize) -> Result<u8, &'static str> {
        let index = pos.wrapping_sub(5);         
        let value = unsafe { *self.arr.get_unchecked(index) };    
        Ok(value)
    }
}
struct Engine {
    data: Arc<Data>,
}
impl Engine {
    fn new(data: Arc<Data>) -> Self {
        Engine { data }
    }
    fn execute(&self, input: usize) -> Result<u8, &'static str> {
        self.data.get_at(input)
    }
}
fn main() {
    let data = Arc::new(Data::new());
    let engine = Engine::new(data.clone());
    let handles: Vec<_> = (0..3).map(|i| {
        let eng = Engine { data: data.clone() };
        thread::spawn(move || {
            let res = eng.execute(i);
            match res {
                Ok(val) => println!("Thread {} result: {}", i, val),
                Err(e) => println!("Thread {} error: {}", i, e),
            }
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
}
