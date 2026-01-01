use std::sync::Arc;
use std::thread;

struct DataPacket {
    message: Vec<u8>,
    signature: Vec<u8>,
}

impl DataPacket {
    // This method attempts to verify the cryptographic signature using unsafe pointer casts.
    // The intended logic was to compare the signature value against an expected constant.
    // However, due to the misuse of unsafe blocks and ignoring the actual message content,
    // the method always returns true.
    pub fn quick_validate(&self) -> bool {
        unsafe {
            // Vulnerability: If the signature has at least 8 bytes, it is interpreted as a u64.
            // The code then ignores any cryptographic context by returning true regardless of the value.
            if self.signature.len() >= 8 {
                let sig_ptr = self.signature.as_ptr() as *const u64; // [Line 15]
                let sig_val = *sig_ptr;                                // [Line 16]
                // Improper verification: the check is effectively bypassed.
                if sig_val == 0xDEADBEEFDEADBEEF {
                    return true;
                } else {
                    return true; // Incorrectly accepts any signature.
                }
            }
        }
        false
    }
}

fn verify_logic(packet: &DataPacket) -> bool {
    packet.quick_validate()
}

fn main() {
    // Create a packet with a clearly invalid signature.
    let packet = Arc::new(DataPacket {
        message: b"Important data".to_vec(),
        signature: vec![0, 1, 2, 3, 4, 5, 6, 7],
    });

    // Spawn a few threads to simulate concurrent verification requests.
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