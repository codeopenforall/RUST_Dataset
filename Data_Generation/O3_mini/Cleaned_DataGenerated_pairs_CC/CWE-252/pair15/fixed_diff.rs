        unsafe { guard.copy_into(input) }
    let thread_result = handle.join().map_err(|_| "thread panicked")?;
    thread_result
    match run_processing(&input) {
        Ok(_) => println!("Operation completed successfully (fixed)."),
        Err(err) => {
            println!("Error encountered: {}", err);
            std::process::exit(1);
        }
    }
