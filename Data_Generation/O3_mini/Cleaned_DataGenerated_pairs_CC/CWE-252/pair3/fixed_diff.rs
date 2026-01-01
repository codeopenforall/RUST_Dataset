use std::io::{self, Write};
fn run() -> io::Result<()> {
    let mut handles = vec![];
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || -> io::Result<()> {
            unsafe {
                let path = if env::var("SIM_FAILURE").is_ok() {
                    "invalid_dir/log.txt"
                } else {
                    "log.txt"
                };
                let mut file = OpenOptions::new().write(true).create(true).open(path)?;
                file.write_all(format!("Thread {} writing\n", i).as_bytes())?;
            }
            let mut num = counter.lock().unwrap();
            *num += 1;
            Ok(())
        });
        handles.push(handle);
    for handle in handles {
        handle.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panicked"))??;
    }
    Ok(())
    if let Err(e) = run() {
        eprintln!("Critical error: {}", e);
        std::process::exit(1);
    }
