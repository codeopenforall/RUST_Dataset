use std::env;
struct Packet {
    payload: Vec<u8>,
}
impl Packet {
    fn parse(data: &[u8]) -> Result<Packet, &'static str> {
        if data.len() < 4 {
            return Err("Input too short");
        }
        let payload_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        unsafe {
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
