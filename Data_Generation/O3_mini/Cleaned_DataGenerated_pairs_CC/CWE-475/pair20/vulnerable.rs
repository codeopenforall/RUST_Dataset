use std::thread;
struct Container {
    payload: Vec<u8>,
}
impl Container {
    fn compute(&self) -> Option<u32> {
        let mut sum = 0u32;
        unsafe {
            let ptr = self.payload.as_ptr() as *const u32;
            for i in 0..3 {
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}
fn run() {
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let container = Container { payload: data };
    let handle = thread::spawn(move || {
        container.compute()
    });
    let result = handle.join().unwrap();
    println!("Computed sum: {:?}", result);
}
fn main() {
    run();
}
