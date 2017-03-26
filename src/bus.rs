
pub trait Bus {
    // on the SH2, a word is 32 bits wide
    fn read_byte(&self, addr: u32) -> u8;
    fn read_word(&self, addr: u32) -> u16;
    fn read_long(&self, addr: u32) -> u32;
    fn write_byte(&mut self, addr: u32, val: u8);
    fn write_word(&mut self, addr: u32, val: u16);
    fn write_long(&mut self, addr: u32, val: u32);
}
