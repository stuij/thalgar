// emulation for the SH7606 microcontroller non-cpu parts

use bus::Bus;

struct Regs {
    // INTC io regs             access
    //                            size
    _ipra:    u16, // 0xFFFFFEE2  8, 16
    _iprb:    u16, // 0xFFFFFE60  8, 16
    _vcra:    u16, // 0xFFFFFE62  8, 16
    _vcrb:    u16, // 0xFFFFFE64  8, 16
    vcrc:    u16, // 0xFFFFFE66  8, 16
    _vcrd:    u16, // 0xFFFFFE68  8, 16
    _vcrwdt:  u16, // 0xFFFFFEE4  8, 16
    _vcrdiv:  u32, // 0xFFFFFF0C     32
    _vcrdma0: u32, // 0xFFFFFFA0     32
    _vcrdma1: u32, // 0xFFFFFFA8     32
    _icr:     u16, // 0xFFFFFEE0  8, 16
}

impl Regs {
    fn new() -> Regs {
        Regs {
            _ipra:        0x0000,
            _iprb:        0x0000,
            _vcra:        0x0000,
            _vcrb:        0x0000,
            vcrc:        0x0000,
            _vcrd:        0x0000,
            _vcrwdt:      0x0000,
            _vcrdiv:  0xdeadbeef,
            _vcrdma0: 0xdeadbeef,
            _vcrdma1: 0xdeadbeef,
            _icr:         0x0000,
        }
    }

    // TODO: reset fn
}

pub struct Sh7604Mem<U: Bus> {
    regs: Regs,
    pub user: U,
}

impl<U: Bus> Sh7604Mem<U> {
    pub fn new(user_mem: U) -> Sh7604Mem<U> {
        Sh7604Mem {
            regs: Regs::new(),
            user: user_mem,
        }
    }

    // TODO: reset fn
}


impl<U: Bus> Bus for Sh7604Mem<U> {
    // byte access
    fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            0xe0000000 ... 0xffffffff => {
                match addr {
                    _ => panic!("sh7604 read_byte: {:#010x} not (yet) mapped",
                                addr)
                }
            },
            _ => self.user.read_byte(addr)
        }
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 write_byte: no private mem mapped yet")
            },
            _ => self.user.write_byte(addr, val)
        };
    }

    // word access
    fn read_word(&self, addr: u32) -> u16 {
        match addr {
            0xe0000000 ... 0xffffffff => {
                match addr {
                    0xfffffe66 => self.regs.vcrc,
                    _ => panic!("sh7604 read_word: {:#010x} not (yet) mapped",
                                addr)
                }
            },
            _ => self.user.read_word(addr)
        }
    }

    fn write_word(&mut self, addr: u32, val: u16) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 write_word: no private mem mapped yet")
            },
            _ => self.user.write_word(addr, val)
        };
    }

    // long access
    fn read_long(&self, addr: u32) -> u32 {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 read_long: no private mem mapped yet")
            },
            _ => self.user.read_long(addr)
        }
    }

    fn write_long(&mut self, addr: u32, val: u32) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 write_long: no private mem mapped yet")
            },
            _ => self.user.write_long(addr, val)
        };
    }
}
