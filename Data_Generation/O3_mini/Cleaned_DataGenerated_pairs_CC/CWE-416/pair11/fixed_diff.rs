    memory: Box<i32>,
    fn obtain(&self) -> i32 {
        *self.memory
    Resource { memory: boxed }
    let val = resource.obtain();
    println!("Value: {}", val);
