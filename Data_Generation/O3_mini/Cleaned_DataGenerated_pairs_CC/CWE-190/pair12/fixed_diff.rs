trait CheckedOps {
    fn compute(&self, a: u64, b: u64) -> u64;
impl CheckedOps for Engine {
    fn compute(&self, a: u64, b: u64) -> u64 {
        a.checked_mul(b).expect("Multiplication overflow")
        engine_clone.compute(a, b)
    handler.join().unwrap()
