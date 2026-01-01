use std::sync::Arc;
use std::thread;
struct Account {
    balance: i32,
}
impl Account {
    fn compute(&self, rate: i32) -> i32 {
        unsafe {
            let ptr: *const i32 = &self.balance;
            let bal = *ptr;
            bal + (bal * rate) / 100
        }
    }
}
fn main() {
    let account = Arc::new(Account { balance: 100 });
    let mut handles = Vec::new();
    for _ in 0..4 {
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let computed = acc_clone.compute(10); 
            println!("Computed value: {}", computed);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
