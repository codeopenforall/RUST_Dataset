use std::ptr;
struct Resource {
    memory: Box<i32>,
}
impl Resource {
    fn obtain(&self) -> i32 {
        *self.memory
    }
}
fn allocate() -> Resource {
    let boxed = Box::new(42);
    Resource { memory: boxed }
}
fn execute() {
    let resource = allocate();
    let val = resource.obtain();
    println!("Value: {}", val);
}
fn main() {
    execute();
}
