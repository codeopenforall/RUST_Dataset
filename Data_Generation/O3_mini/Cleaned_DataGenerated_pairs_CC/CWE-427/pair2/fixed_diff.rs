fn run_protected() {
    let allowed_commands = ["echo", "safe"];
    if !allowed_commands.contains(&command_input.as_str()) {
        panic!("Command not allowed");
    }
    let shell = command_input.as_str();
    let secure_path = "/usr/bin:/bin";
            .env("PATH", secure_path)
            .arg("fixed")
    run_protected();
