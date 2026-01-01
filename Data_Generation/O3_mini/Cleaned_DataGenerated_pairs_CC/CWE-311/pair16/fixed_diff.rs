    fn xor_enc(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }
        let encrypted = Service::xor_enc(&copied, 0xAA);
        encrypted.iter().map(|b| format!("{:02x}", b)).collect()
