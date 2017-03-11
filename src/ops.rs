#[macro_use]

// instruction format macros
macro_rules! nm_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let regs = ($op & 0xfff) >> 4;
        let rn = regs >> 0x4;
        let rm = regs & 0xf;
        $this.$fun($bus, rm, rn);
    }
}

macro_rules! do_op {
    ($this:ident, $bus:expr, $op:expr) => {
        match $op >> 12 {
            // we're starting with the most significant nibble
            0b0010 => {
                // least significant nibble
                match $op & 0xf {
                    0b0110 => {
                        nm_format!($this, $bus, $op, movl);
                    },
                    _ => panic!("did not recognize least significant nibble {:#06b} of op {:#06x}", $op & 0xF, $op)
                }
            },
            _ => panic!("did not recognize most significant nibble {:#06b} of op {:#06x}", $op >> 12, $op)
        }
    }
}
