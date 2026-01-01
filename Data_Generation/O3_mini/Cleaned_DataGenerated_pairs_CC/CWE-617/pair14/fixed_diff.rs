    let safe_input = if input < THRESHOLD { input } else { THRESHOLD - 1 };
    let shared = Arc::new(Mutex::new(unsafe { Data::init(safe_input) }));
        data.operate();
    data_main.operate();
