pub struct Memory {
    data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; 65536],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}