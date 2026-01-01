//////////////////////////////////////////////////////////////
// Corrected Code (Safe Packet Reader)
// This corrected version explicitly validates that the input slice has at least
// (header + payload length) bytes before attempting to extract the payload.
// It returns an appropriate error if the check fails, eliminating the vulnerability.
//////////////////////////////////////////////////////////////
use std::env;
use std::process;

struct Packet {
    payload: Vec<u8>,
}

impl Packet {
    fn parse(data: &[u8]) -> Result<Packet, &'static str> {
        if data.len() < 4 {
            return Err("Input too short");
        }
        let payload_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        // FIX: Validate that the full payload is present.
        if data.len() < 4 + payload_len {
            return Err("Payload length exceeds available data");
        }
        let payload = data[4..4 + payload_len].to_vec();
        Ok(Packet { payload })
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
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}