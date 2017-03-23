
pub trait Bus {
    // on the SH2, a word is 32 bits wide
    fn read_word(&self, addr: u32) -> u16;
    fn read_long(&self, addr: u32) -> u32;
    fn write_long(&mut self, addr: u32, val: u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestBus {
        addr: [u16; 2]
    }


    impl Bus for TestBus {
        #[allow(unused_variables)]
        fn read_word(&self, addr: u32) -> u16 {
            self.addr[0]
        }

        #[allow(unused_variables)]
        fn read_long(&self, addr: u32) -> u32 {
            (self.addr[0] as u32) << 16 |
            self.addr[1] as u32
        }

        #[allow(unused_variables)]
        fn write_long(&mut self, addr: u32, val: u32) {
            self.addr[0] = (val >> 16) as u16;
            self.addr[1] = (val & 0xFFFF) as u16;
        }
    }

    #[test]
    fn read_a_word() {
        let bus = TestBus { addr: [0xffee, 0xddcc] };
        assert_eq!(bus.read_word(0), 0xffee);
    }

    #[test]
    fn write_a_long() {
        let mut bus = TestBus { addr: [0x00, 0x11] };
        bus.write_long(0, 0xffeeddcc);
        assert_eq!(bus.addr, [0xffee, 0xddcc]);
    }
}
