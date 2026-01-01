//////////////////////////////////////////////////////////////
// Vulnerable Code (Unsafe Packet Reader)
// This code accepts an input as a sequence of bytes where the first 4 bytes
// encode the payload length as a little‐endian u32. It then unsafely reads the
// payload using from_raw_parts without checking that the provided slice is long enough.
// This may lead to reading beyond the allocated buffer when a user supplies a
// malicious input where the length field does not match the actual data size.
//////////////////////////////////////////////////////////////
use std::env;

struct Packet {
    payload: Vec<u8>,
}

impl Packet {
    // Improper input validation: no check is made to ensure that data.len() >= 4 + payload_len.
    fn parse(data: &[u8]) -> Result<Packet, &'static str> {
        if data.len() < 4 {
            return Err("Input too short");
        }
        // The user-controlled header field is converted into a payload length.
        let payload_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        // Unsafe block: No verification that data has enough bytes.
        // Returns a Packet even if payload_len is greater than data.len()-4, which is a vulnerability.
        unsafe {
            // Vulnerability occurs because this does not check that (data.len() - 4) >= payload_len.
            let ptr = data.as_ptr().add(4);
            let slice = std::slice::from_raw_parts(ptr, payload_len);
            Ok(Packet { payload: slice.to_vec() })
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_data>", args[0]);
        return;
    }
    let input = args[1].clone();
    let data = input.into_bytes();
    match Packet::parse(&data) {
        Ok(packet) => println!("Payload: {:?}", packet.payload),
        Err(e) => eprintln!("Error: {}", e),
    }
}