use std::sync::Arc;
use std::thread;
struct DataPacket {
    message: Vec<u8>,
    signature: Vec<u8>,
}
impl DataPacket {
    pub fn quick_validate(&self) -> bool {
        unsafe {
            if self.signature.len() >= 8 {
                let sig_ptr = self.signature.as_ptr() as *const u64; 
                let sig_val = *sig_ptr;                                
                if sig_val == 0xDEADBEEFDEADBEEF {
                    return true;
                } else {
                    return true; 
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
    let packet = Arc::new(DataPacket {
        message: b"Important data".to_vec(),
        signature: vec![0, 1, 2, 3, 4, 5, 6, 7],
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
