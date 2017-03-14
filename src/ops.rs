#[macro_use]

// instruction format macros
macro_rules! n_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let rn = ($op & 0x0f00) >> 8;
        $this.$fun($bus, rn);
    }
}

macro_rules! nm_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let regs = ($op & 0x0ff0) >> 4;
        let rn = regs >> 0x4;
        let rm = regs & 0xf;
        $this.$fun($bus, rm, rn);
    }
}


// STS.L PR, @–Rn

macro_rules! do_op {
    ($this:ident, $bus:expr, $op:expr) => {
        match $op >> 12 {
            // we're starting with the most significant nibble
            0b0010 => {
                // least significant nibble
                match $op & 0xf {
                    0b0110 => { nm_format!($this, $bus, $op, movl); },
                    _ => panic!("did not recognize least significant nibble \
                                 {:#06b} of op {:#06x}", $op & 0xF, $op)
                }
            },
            0b0100 => {
                if ($op & 0xf) == 0xf {
                    panic!("please implement MAC.W @Rm+,@Rn+")
                } else {
                    match $op & 0xff {
                        0b00100010 => { n_format!($this, $bus, $op, stsl_pr); },
                        _ => panic!("did not recognize least significant byte \
                                     {:#010b} of op {:#06x}", $op & 0xFF, $op)
                    }
                }
            },
            _ => panic!("did not recognize most significant nibble \
                         {:#06b} of op {:#06x}", $op >> 12, $op)
        }
    }
}
