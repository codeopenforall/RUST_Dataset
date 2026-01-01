/*
   Test Oracle Function:

   This test function is independent of the programs above.
   It targets the behavior of the extraction function using a known triggering input.
   For the problematic version, supplying an offset and count that exceed the buffer
   length may trigger undefined behavior or cause a panic. For the corrected version,
   the function should safely adjust the output length.

   The test uses a fixed input buffer "ABCDEFGHIJKLMNOPQRSTUVWXYZ".
   Calling process() with offset=20 and count=10:
     - In the problematic version, this input is expected to lead to a failure (e.g., panic or reading invalid data).
     - In the corrected version, the function will return "UVWXYZ" (6 characters).

   The oracle asserts that the output of the extraction is exactly "UVWXYZ".
*/

#[cfg(test)]
mod tests {
    use super::Holder;

    #[test]
    fn test_extraction() {
        let data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let holder = Holder::new(data).expect("Initialization failed");
        let result = holder.process(20, 10);
        // Expected result: Since the buffer length is 26, from offset 20 only 6 bytes are valid.
        assert_eq!(result, "UVWXYZ", "The extracted substring did not match the expected safe output.");
    }
}