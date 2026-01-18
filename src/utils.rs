pub fn get_le_16_bytes(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(|i| i.to_le_bytes()).collect()
}

pub fn get_be_16_bytes(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(|i| i.to_be_bytes()).collect()
}
