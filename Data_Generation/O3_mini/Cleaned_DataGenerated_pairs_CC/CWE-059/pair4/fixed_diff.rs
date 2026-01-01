    let allowed_dir = Path::new("/safe/directory");
    let given_path = Path::new(&file_path);
    let resolved_path = fs::canonicalize(given_path).expect("Failed to resolve file path");
    if !resolved_path.starts_with(allowed_dir) {
        panic!("Access denied: File is outside the allowed directory");
    }
    if resolved_path.is_file() {
        let mut file = File::open(&resolved_path).expect("Cannot open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");
        println!("File contents: {}", contents);
    } else {
        println!("Not a regular file: {:?}", resolved_path);
