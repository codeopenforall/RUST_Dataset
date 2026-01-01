        let required = mem::size_of::<&str>(); 
        String::from_utf8_lossy(&buffer).into()
