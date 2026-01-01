        if xml.contains("<!ENTITY") || xml.contains("&ext;") {
            return Err("External entity resolution has been disabled".to_string());
        }
        let res_clone = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let processed = content.replace("&amp;", "&");
            let mut guard = res_clone.lock().unwrap();
            *guard = processed;
        });
        handle.join().map_err(|_| "Thread panicked")?;
<data>&amp;</data>"#;
