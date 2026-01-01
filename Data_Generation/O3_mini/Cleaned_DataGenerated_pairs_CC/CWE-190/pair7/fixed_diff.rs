        a.checked_mul(b).ok_or("Overflow detected")
        assert!(result.is_err(), "Overflow was not detected safely");
