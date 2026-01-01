const MAX_TASKS: usize = 100; 
    if num > MAX_TASKS {
        return Err("Error: Too many tasks requested");
    }
        args[1].parse().unwrap_or(MAX_TASKS)
        MAX_TASKS
    if let Err(e) = execute(tasks) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
