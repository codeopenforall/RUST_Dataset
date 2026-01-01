        if val < 0 {
            return Err("negative value not allowed");
        }
        Ok(())
        h.modify(-10).expect("update failed due to negative value");
    let _ = handle.join();
