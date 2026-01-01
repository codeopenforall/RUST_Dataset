use std::rc::Rc;
#[derive(Debug)]
struct Data {
    value: i32,
}
trait Action {
    fn execute(&self) -> i32;
}
struct Handler {
    data: Rc<Data>,
}
impl Action for Handler {
    fn execute(&self) -> i32 {
        self.data.value
    }
}
pub fn compute() -> i32 {
    let data_rc = Rc::new(Data { value: 42 });
    let handler = Handler { data: Rc::clone(&data_rc) };
    let result = handler.execute();
    result
}
fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}
