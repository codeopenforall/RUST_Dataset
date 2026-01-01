use std::io::Write;
fn run() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                unsafe {
                    let path = if env::var("SIM_FAILURE").is_ok() {
                        "invalid_dir/log.txt"
                    } else {
                        "log.txt"
                    };
                    let file_result = OpenOptions::new().write(true).create(true).open(path);
                    if let Ok(mut file) = file_result {
                        let _ = file.write_all(format!("Thread {} writing\n", i).as_bytes());
                    }
                }
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    run();
