trait Arithmetic {
    fn operate(&self, numerator: i32, denominator: i32) -> i32;
struct Engine;
impl Arithmetic for Engine {
    fn operate(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            let ptr = &denominator as *const i32;
            let div = *ptr;
            numerator / div   
fn process(numerator: i32, denominator: i32) -> i32 {
    let engine = Engine;
        engine.operate(numerator, denominator)
    process(numerator, denominator)
    let result = process(10, 0);
