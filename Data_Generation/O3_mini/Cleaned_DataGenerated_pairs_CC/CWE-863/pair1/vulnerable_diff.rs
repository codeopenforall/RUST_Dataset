static mut LIMIT: usize = 1000;
        let _limit = unsafe { LIMIT };
            let handle = {
                unsafe {
                    thread::spawn(move || {
                        task.execute();
                    })
                }
            };
    manager.allocate(10).expect("Allocation failed unexpectedly");
    manager.join_all();
    println!("Execution completed in vulnerable build.");
