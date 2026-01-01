use std::thread;
trait Execute {
    fn execute(&self);
}
struct Runner;
impl Execute for Runner {
    fn execute(&self) {
        let handle = thread::spawn(|| {
            unsafe {
                let num_ptr = &10 as *const i32;
                let value = *num_ptr; 
                if value == 10 {
                    panic!("Thread encountered an unrecovered panic");
                }
            }
        });
        handle.join().unwrap();
    }
}
pub fn run_logic() {
    let runner = Runner;
    runner.execute();
}
fn main() {
    run_logic();
    println!("Execution completed (this line is never reached in failure cases)");
}
