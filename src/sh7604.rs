// emulation for the SH7606 microcontroller non-cpu parts

use bus::Bus;

struct Regs {
    //                           access
    //                             size

    // FRT (Free Running Timer) regs
    tier:     u8,  // 0xFFFFFE10      8
    ftcsr:     u8, // 0xFFFFFE11      8
    _frc_h:    u8, // 0xFFFFFE12      8
    _frc_l:    u8, // 0xFFFFFE13      8
    ocra_h:    u8, // 0xFFFFFE14      8
    ocra_l:    u8, // 0xFFFFFE15      8
    ocrb_h:    u8, // 0xFFFFFE14      8
    ocrb_l:    u8, // 0xFFFFFE15      8
    tcr:       u8, // 0xFFFFFE16      8
    tocr:      u8, // 0xFFFFFE17      8
    _icr_h:    u8, // 0xFFFFFE18      8
    _icr_l:    u8, // 0xFFFFFE19      8

    // INTC io regs
    _ipra:    u16, // 0xFFFFFEE2  8, 16
    iprb:     u16, // 0xFFFFFE60  8, 16
    _vcra:    u16, // 0xFFFFFE62  8, 16
    _vcrb:    u16, // 0xFFFFFE64  8, 16
    vcrc:     u16, // 0xFFFFFE66  8, 16
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
            // FRT
            tier:           0x01,
            ftcsr:          0x00,
            _frc_h:         0x00,
            _frc_l:         0x00,
            ocra_h:         0xff,
            ocra_l:         0xff,
            ocrb_h:         0xff,
            ocrb_l:         0xff,
            tcr:            0x00,
            tocr:           0xe0,
            _icr_h:         0x00,
            _icr_l:         0x00,

            // INTC
            _ipra:        0x0000,
            iprb:         0x0000,
            _vcra:        0x0000,
            _vcrb:        0x0000,
            vcrc:         0x0000,
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
                    // curious but true: these two addresses below
                    // are shared between registers
                    0xfffffe14 => {
                        if self.regs.tocr & 0x10 == 0 { self.regs.ocra_h }
                        else { self.regs.ocrb_h }
                    },
                    0xfffffe15 => {
                        if self.regs.tocr & 0x10 == 0 { self.regs.ocra_l }
                        else { self.regs.ocrb_l }
                    },
                    0xfffffe16 => self.regs.tcr,
                    _ => panic!("sh7604 read_byte: {:#010x} not (yet) mapped",
                                addr)
                }
            },
            _ => self.user.read_byte(addr & 0xdfffffff)
        }
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                match addr {
                    0xfffffe10 => self.regs.tier = val,
                    0xfffffe11 => self.regs.ftcsr = val,
                    0xfffffe14 => {
                        if self.regs.tocr & 0x10 == 0 { self.regs.ocra_h = val }
                        else { self.regs.ocrb_h = val }
                    },
                    0xfffffe15 => {
                        if self.regs.tocr & 0x10 == 0 { self.regs.ocra_l = val }
                        else { self.regs.ocrb_l = val }
                    },
                    0xfffffe16 => self.regs.tcr = val,
                    _ => panic!("sh7604 write_byte: {:#010x} not (yet) mapped",
                           addr)
                }
            },
            _ => self.user.write_byte(addr & 0xdfffffff, val)
        };
    }

    // word access
    fn read_word(&self, addr: u32) -> u16 {
        match addr {
            0xe0000000 ... 0xffffffff => {
                match addr {
                    0xfffffe60 => self.regs.iprb,
                    0xfffffe66 => self.regs.vcrc,
                    _ => panic!("sh7604 read_word: {:#010x} not (yet) mapped",
                                addr)
                }
            },
            _ => self.user.read_word(addr & 0xdfffffff)
        }
    }

    fn write_word(&mut self, addr: u32, val: u16) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                match addr {
                    0xfffffe60 => self.regs.iprb = val,
                    0xfffffe66 => self.regs.vcrc = val,
                    _ => panic!("sh7604 write_word: {:#010x} not (yet) mapped",
                                addr)
                }
            },
            _ => self.user.write_word(addr & 0xdfffffff, val)
        };
    }

    // long access
    fn read_long(&self, addr: u32) -> u32 {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 read_long: no private mem mapped yet")
            },
            _ => self.user.read_long(addr & 0xdfffffff)
        }
    }

    fn write_long(&mut self, addr: u32, val: u32) {
        match addr {
            0xe0000000 ... 0xffffffff => {
                panic!("sh7604 write_long: {:#010x} not (yet) mapped",
                       addr)
            },
            _ => self.user.write_long(addr & 0xdfffffff, val)
        };
    }
}
