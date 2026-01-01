//////////////////////////////////////////////////////////////
// Oracle Test Function
//
// This test verifies the correct behavior of the code handling
// the copying of a string into a buffer. When provided with 50 'A's,
// the correct checksum should equal (50 * 65) mod 256, which is 174.
// For the vulnerable version, the checksum is likely different
// due to memory corruption from the buffer overflow.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // Expected checksum: 50 * ASCII('A') mod 256 = 3250 % 256 = 174.
    fn expected_checksum() -> u8 {
        50u8.wrapping_mul(65)
    }

    #[test]
    fn test_checksum() {
        let comm = Communicator;
        let handle = thread::spawn(move || {
            let data = "A".repeat(50);
            let buf = comm.transmit(&data);
            // For the fixed version, only the first 50 bytes (excluding terminator) are summed.
            buf[..50].iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
        });
        let checksum = handle.join().unwrap();
        assert_eq!(checksum, expected_checksum(), "Checksum does not match expected value");
    }
}