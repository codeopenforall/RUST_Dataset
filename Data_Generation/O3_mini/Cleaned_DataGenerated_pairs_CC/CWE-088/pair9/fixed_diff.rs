    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn({
        let user_input = input.to_owned();
        move || {
            let output = Command::new("echo")
                .arg(user_input)
    });
    match rx.recv() {
        Ok(Ok(o)) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Ok(Err(e)) => Err(format!("Command error: {:?}", e)),
        Err(e) => Err(format!("Channel error: {:?}", e)),
