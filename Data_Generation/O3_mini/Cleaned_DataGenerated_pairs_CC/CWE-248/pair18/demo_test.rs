#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::panic;

    // Declaration for the external function.
    extern "C" {
        fn service(input: *const c_char) -> i32;
    }

    #[test]
    fn validate_service_error_handling() {
        let test_input = CString::new("oracle_test").unwrap();
        // Wrap the call in catch_unwind to detect if a panic occurs.
        let result = panic::catch_unwind(|| unsafe { service(test_input.as_ptr()) });
        // For the fixed implementation, the panic is caught internally and an error code is returned.
        // Thus, catch_unwind should see Ok(...) with the expected error code.
        assert!(result.is_ok(), "Service unexpectedly panicked without proper handling.");
        let ret_code = result.unwrap();
        assert_eq!(ret_code, -1, "Service did not return the expected error code.");
    }
}