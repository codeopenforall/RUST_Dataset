///////////////////////////////////////////////////////////////
// This test oracle function is used to verify the behavior of the 
// implementations. It is designed to ensure that an out-of-bounds
// update operation results in a panic (which is the correct and safe
// behavior in the fixed version). The vulnerable version (which lacks
// bounds check) is expected to not panic and thus will cause this test
// to fail.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn oracle() {
        // Create a buffer with 10 elements.
        let buf = Buffer::new(10);
        // Use catch_unwind to evaluate if the update operation panics.
        let result = panic::catch_unwind(|| {
            // Attempting an update with an invalid index.
            // The fixed implementation should panic.
            buf.update(15, 99);
        });
        assert!(result.is_err(), "Update did not panic on out-of-bounds access");
    }
}