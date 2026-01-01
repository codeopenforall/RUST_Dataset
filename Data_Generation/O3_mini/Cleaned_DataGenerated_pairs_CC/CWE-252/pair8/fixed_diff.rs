    fn run(&self, data: &str) -> io::Result<()> {
        let mut handles = Vec::new();
        for _ in 0..2 {
            let handle = thread::spawn(move || -> io::Result<()> {
                    Self::update_file(&msg_clone)?
                Ok(())
            });
            handles.push(handle);
        for handle in handles {
            handle.join()
                  .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread join failed"))??
        }
        Ok(())
        fs::write("log.txt", msg)
    if let Err(e) = processor.run(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
