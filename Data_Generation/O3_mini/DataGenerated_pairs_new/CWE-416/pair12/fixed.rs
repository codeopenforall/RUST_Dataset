use std::thread;

trait Arithmetic {
    fn compute(&self) -> i32;
}

struct Container {
    data: i32,
}

impl Arithmetic for Container {
    fn compute(&self) -> i32 {
        // Simple computation: add one to the internal data.
        self.data + 1
    }
}

fn process() -> i32 {
    // Allocate memory for the resource.
    let resource = Box::new(Container { data: 99 });
    // Use the safe abstraction to compute the result; no unsafe raw pointer conversions.
    resource.compute()
}

fn main() {
    // Call the process function to obtain a safe computation result.
    let val = process();
    println!("Result: {}", val);

    // Spawn a thread that uses the resource safely.
    let handle = thread::spawn(|| {
        let boxed = Box::new(Container { data: 50 });
        let result = boxed.compute();
        println!("Thread result: {}", result);
    });
    handle.join().unwrap();
    thread::sleep(std::time::Duration::from_millis(50));
}