trait Operations {
    unsafe fn compute(&self, a: u64, b: u64) -> u32;
impl Operations for Engine {
    unsafe fn compute(&self, a: u64, b: u64) -> u32 {
        let product = a * b;
        product as u32
        unsafe { engine_clone.compute(a, b) }
    let result_u32 = handler.join().unwrap();
    result_u32 as u64
