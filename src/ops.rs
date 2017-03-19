#[macro_use]

// instruction format macros

// PC relative 12 bits of displacement
macro_rules! d12_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let disp = ($op & 0x0fff) as u32;
        $this.$fun($bus, disp);
    }
}


// 1 register
macro_rules! n_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let rn = (($op & 0x0f00) >> 8) as usize;
        $this.$fun($bus, rn);
    }
}

// 2 registers
macro_rules! nm_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let regs = ($op & 0x0ff0) >> 4;
        let rn = (regs >> 0x4) as usize;
        let rm = (regs & 0xf) as usize;
        $this.$fun($bus, rm, rn);
    }
}


// PC relative with displacement
macro_rules! nd8_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let rn = (($op & 0x0f00) >> 0x8) as usize;
        let d = ($op & 0xff) as u32;
        $this.$fun($bus, d, rn);
    }
}

// register + sign extended immediate
macro_rules! ni_format {
    ($this:ident, $bus:expr, $op:expr, $fun:ident) => {
        let rn = (($op & 0x0f00) >> 0x8) as usize;
        let i = $op as i8 as i32 as u32;
        $this.$fun($bus, i, rn);
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
                    0b0110 => { nm_format!($this, $bus, $op, mov_lm); },
                    _ => $this.op_least_significant_nibble_unknown($op)
                }
            },
            0b0100 => {
                if ($op & 0xf) == 0xf {
                    panic!("please implement MAC.W @Rm+,@Rn+")
                } else {
                    match $op & 0xff {
                        0b00100010 => { n_format!($this, $bus, $op, sts_mpr); },
                        _ => $this.op_least_significant_byte_unknown($op)
                    }
                }
            },
            0b0110 => {
                match $op & 0xf {
                    0b0010 => { nm_format!($this, $bus, $op, mov_ll); },
                    _ => $this.op_least_significant_nibble_unknown($op)
                }
            },
            0b1010 => { d12_format!($this, $bus, $op, bra); },
            0b1101 => { nd8_format!($this, $bus, $op, mov_li); },
            0b1110 => { ni_format!($this, $bus, $op, mov_i); },
            _ => $this.op_most_significant_nibble_unknown($op)
        }
    }
}
