use std::convert::TryInto;
use std::sync::Arc;
use std::thread;
struct DataPacket {
    message: Vec<u8>,
    signature: Vec<u8>,
}
fn compute_hash(message: &[u8]) -> u64 {
    let mut hash = 0u64;
    for &b in message {
        hash = hash.wrapping_add(b as u64);
        hash = hash.rotate_left(3);
    }
    hash
}
impl DataPacket {
    pub fn proper_validate(&self) -> bool {
        if self.signature.len() != 8 {
            return false;
        }
        let expected = compute_hash(&self.message);
        let sig_bytes: [u8; 8] = match self.signature[..8].try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        let sig_val = u64::from_le_bytes(sig_bytes);
        expected == sig_val
    }
}
fn verify_logic(packet: &DataPacket) -> bool {
    packet.proper_validate()
}
fn main() {
    let message = b"Important data".to_vec();
    let valid_signature = compute_hash(&message).to_le_bytes().to_vec();
    let packet = Arc::new(DataPacket {
        message,
        signature: valid_signature,
    });
    let mut threads = vec![];
    for _ in 0..4 {
        let pkt = Arc::clone(&packet);
        threads.push(thread::spawn(move || {
            if verify_logic(&pkt) {
                println!("Verification accepted");
            } else {
                println!("Verification rejected");
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
}
