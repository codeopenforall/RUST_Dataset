use std::os::unix::fs::MetadataExt;
    let target = Path::new(&file_path);
    if let Ok(metadata) = fs::symlink_metadata(target) {
        if metadata.file_type().is_symlink() {  
            if let Ok(real_path) = fs::read_link(target) {
                println!("Following symbolic link to: {:?}", real_path);
                unsafe {
                    let mut file = File::open(&real_path).expect("Cannot open real file!");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("Could not read file");
                    println!("File contents: {}", contents);
                }
            }
        } else {
            println!("Regular file detected: {:?}", target);
        }
