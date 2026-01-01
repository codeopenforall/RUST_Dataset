use std::fs;
            if data.contains("<!ENTITY") {
                unsafe {
                    let content = fs::read_to_string("external.txt").unwrap_or_else(|_| String::new());
                    let ptr = content.as_ptr();
                    let len = content.len();
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let result = String::from_utf8_lossy(slice).into_owned();
                    let mut lock = shared.lock().unwrap();
                    *lock = Some(result);
                }
            } else {
                let mut lock = shared.lock().unwrap();
                *lock = Some(String::from("Default content"));
            }
    if let Err(e) = processor.process(xml_input) {
        eprintln!("Processing error: {}", e);
        return;
    }
    if let Some(result) = processor.get_output() {
        println!("Processed output: {}", result);
    } else {
        println!("No output generated.");
