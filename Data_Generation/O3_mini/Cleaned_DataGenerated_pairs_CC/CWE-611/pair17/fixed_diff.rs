        if input.contains("&xxe;") || input.contains("<!ENTITY") {
            Err("External entity resolution is disabled".to_string())
        } else {
            Ok(input.to_string())
    let xml_input = "<data>Safe content only</data>";
