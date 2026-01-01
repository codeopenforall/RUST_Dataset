use std::io::{Write, Result, Error, ErrorKind};
    unsafe fn update(&self, val: u32) -> Result<()> {
        let _guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
        let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
        write!(file, "{}", val)?;
        Ok(())
    fn run(&self, iterations: u32) -> Result<()> {
            let handle = thread::spawn(move || -> Result<()> {
                    let mut guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
                let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
                write!(file, "{}", i)?;
                Ok(())
            th.join().map_err(|_| Error::new(ErrorKind::Other, "thread panicked"))??;
        Ok(())
fn main() -> Result<()> {
        worker.update(100)?;
    worker.run(10)?;
    Ok(())
