use std::ptr;

struct Resource {
    memory: Box<i32>,
}

impl Resource {
    // Safely returns the stored value.
    fn obtain(&self) -> i32 {
        *self.memory
    }
}

fn allocate() -> Resource {
    // Allocate and own the integer safely using Box.
    let boxed = Box::new(42);
    Resource { memory: boxed }
}

fn execute() {
    let resource = allocate();
    let val = resource.obtain();
    // Guaranteed to print the valid value as memory is correctly managed.
    println!("Value: {}", val);
}

fn main() {
    execute();
}