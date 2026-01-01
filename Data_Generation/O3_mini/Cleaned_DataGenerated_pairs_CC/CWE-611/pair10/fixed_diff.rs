        if xml.contains("<!ENTITY") {
            return Err("External entities are not allowed".to_string());
    let handler = XmlHandler::new(false); 
