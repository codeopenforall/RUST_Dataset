        let allowed = ["/usr/lib", "/usr/local/lib"];
        if !allowed.contains(&lib) {
            return Err("Library path not allowed".to_string());
        }
        current.push_str(lib);
    let executor = Executor::new("safe run");
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/lib".to_string());
