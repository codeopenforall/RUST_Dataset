        let offset = calc_offset(idx)?;
                *ptr.add(offset) = val;
    if let Err(e) = mgr.update(5, 100) {
        println!("Error updating index 5: {}", e);
    }
    if let Err(e) = mgr.update(10, 200) {
        println!("Properly rejected update at index 10: {}", e);
    }
