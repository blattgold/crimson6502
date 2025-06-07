pub fn bytes_to_word(lo: u8, hi: u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}