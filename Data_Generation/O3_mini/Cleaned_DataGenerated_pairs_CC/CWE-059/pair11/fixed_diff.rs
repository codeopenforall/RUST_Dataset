    let initial = fs::metadata(path)?;
    if !initial.is_file() {
    let mut file = File::open(path)?;
    let current = file.metadata()?;
    if initial.ino() != current.ino() || initial.dev() != current.dev() {
        return Err(io::Error::new(io::ErrorKind::Other, "File changed (TOCTOU detected)"));
