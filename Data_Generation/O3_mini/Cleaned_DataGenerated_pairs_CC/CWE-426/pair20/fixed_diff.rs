    let abs_path = if cfg!(windows) {
        String::from("C:\\Trusted\\dummy_executable")
    } else {
        String::from("/usr/bin/dummy_executable")
    let loader = Loader { path: abs_path };
