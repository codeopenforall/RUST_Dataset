use std::sync::{Arc, Mutex};
use std::thread;
trait Compute {
    fn compute(&self) -> u32;
}
struct DataHolder {
    data: Vec<u32>,
}
impl Compute for DataHolder {
    fn compute(&self) -> u32 {
        let len = self.data.len();
        if len < 2 {
            return 0;
        }
        let mut total: u32 = 0;
        for i in 0..(len - 1) {
            let first = self.data[i];
            let second = self.data[i + 1];
            total = total.wrapping_add(first).wrapping_add(second);
        }
        total
    }
}
fn spawn_task(holder: Arc<Mutex<DataHolder>>) {
    let handle = thread::spawn(move || {
        let guard = holder.lock().unwrap();
        let sum = guard.compute();
        println!("Computed value: {}", sum);
    });
    handle.join().unwrap();
}
fn main() {
    let holder = Arc::new(Mutex::new(DataHolder { data: vec![1, 2, 3, 4, 5] }));
    spawn_task(holder.clone());
}
