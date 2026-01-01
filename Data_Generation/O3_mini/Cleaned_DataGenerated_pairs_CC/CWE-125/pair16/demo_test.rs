/*
The following test oracle function verifies the proper behavior for the safe version
and detects the vulnerability in the unsafe version. The contract expects that attempting
to read at an index equal to the buffer length (i.e., 5) results in a panic. The safe version
should panic (passing the test), whereas the unsafe version may not reliably panic or cause
undefined behavior (failing the test).
*/
#[cfg(test)]
mod tests {
    use super::DataStore;
    use std::panic;

    #[test]
    fn test_oracle() {
        let store = DataStore::new();
        // Choose an index that is exactly the length of the buffer (out-of-bounds).
        let out_of_bounds_index = store.buffer.len();
        let result = panic::catch_unwind(|| {
            // In the fixed (safe) version, this will panic due to bounds checking.
            // In the vulnerable version, it may not panic and instead produce
            // an undefined value.
            store.fetch(out_of_bounds_index)
        });
        assert!(result.is_err(), "Expected panic when accessing an out-of-bounds index");
    }
}