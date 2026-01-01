use std::thread;
trait Execute {
    fn execute(&self) -> Result<(), ()>;
}
struct Runner;
impl Execute for Runner {
    fn execute(&self) -> Result<(), ()> {
        let handle = thread::spawn(|| {
            unsafe {
                let num_ptr = &10 as *const i32;
                let value = *num_ptr;
                if value == 10 {
                    panic!("Thread panic captured safely");
                }
            }
        });
        match handle.join() {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Recovered from thread panic: {:?}", err);
                Ok(())
            }
        }
    }
}
pub fn run_logic() -> Result<(), ()> {
    let runner = Runner;
    runner.execute()
}
fn main() {
    if run_logic().is_ok() {
        println!("Execution completed safely");
    } else {
        println!("Execution encountered errors");
    }
}
