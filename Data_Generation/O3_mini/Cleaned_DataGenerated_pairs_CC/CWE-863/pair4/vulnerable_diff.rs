static mut TASK_COUNT: usize = 0;
                unsafe {
                    TASK_COUNT += 1;
                }
            let _ = handle.join();
    if let Err(e) = service.execute_request(1000) {
        eprintln!("Error: {}", e);
    } else {
        unsafe {
            println!("Total tasks spawned: {}", TASK_COUNT);
        }
