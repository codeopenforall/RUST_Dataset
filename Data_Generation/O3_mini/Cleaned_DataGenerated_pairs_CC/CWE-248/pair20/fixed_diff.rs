    fn execute(&self) -> Result<(), ()>;
    fn execute(&self) -> Result<(), ()> {
                let value = *num_ptr;
                    panic!("Thread panic captured safely");
        match handle.join() {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Recovered from thread panic: {:?}", err);
                Ok(())
            }
        }
pub fn run_logic() -> Result<(), ()> {
    runner.execute()
    if run_logic().is_ok() {
        println!("Execution completed safely");
    } else {
        println!("Execution encountered errors");
    }
