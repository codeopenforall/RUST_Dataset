        unsafe {
            let result = (a as u64 * b as u64) as u32; 
            Ok(result)
        }
        assert!(result.is_err(), "Overflow was not detected: {:?}", result);
