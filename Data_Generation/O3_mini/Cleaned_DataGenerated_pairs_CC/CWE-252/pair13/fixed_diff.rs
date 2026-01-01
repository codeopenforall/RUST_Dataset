fn run_update(resource: &Arc<Resource>, input: &[u8]) -> Result<(), &'static str> {
            res_clone.write_data(input)
    let thread_result = handle.join().map_err(|_| "Thread panicked")?;
    thread_result
    match run_update(resource, input) {
        Ok(()) => true,
        Err(_) => false,
    }
    if !outcome {
        eprintln!("Update failed due to invalid input.");
        std::process::exit(1);
    }
