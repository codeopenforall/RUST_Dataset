    Command::new("echo")
        .arg(input)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
