//////////////////////////////
// Test Oracle Function
//////////////////////////////

// This test oracle is designed to fail for the vulnerable implementation and pass for the corrected one.
// It creates a Packet with "Hello" as the content. The expected serialized length is 4 bytes for the u32 header plus 5 bytes for "Hello", totaling 9 bytes.

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // Stub definition for Packet and pack function will be linked against either vulnerable or corrected implementations.
    // Ensure that the implementation under test exposes the same interface.

    #[test]
    fn test_packet_pack_length() {
        let pkt = Packet {
            tag: 0xDEADBEEF,
            content: String::from("Hello"),
        };

        unsafe {
            let buf = pkt.pack();
            // The header is a u32 (4 bytes) and the content "Hello" is 5 bytes.
            let expected_length = mem::size_of_val(&pkt.tag) + pkt.content.len();
            // The vulnerable code incorrectly calculates the content size, leading to an unexpected total length.
            assert_eq!(buf.len(), expected_length, "Packed buffer length mismatch!");
        }
    }
}