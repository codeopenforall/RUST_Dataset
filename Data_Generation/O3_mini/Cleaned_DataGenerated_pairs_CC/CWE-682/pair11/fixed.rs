use std::sync::Arc;
use std::thread;
struct Transaction {
    amount: u32,
    discount: u32, 
}
impl Transaction {
    pub unsafe fn apply(&self) -> u32 {
        self.amount - ((self.amount * self.discount) / 100)
    }
}
fn main() {
    let transaction = Transaction { amount: 100, discount: 20 }; 
    let shared = Arc::new(transaction);
    let mut handles = vec![];
    for _ in 0..5 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                clone.apply()
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Net amount: {}", result); 
    }
}
