#[macro_use]

// instruction format macros

// PC relative 8 bits of displacement
macro_rules! d8_format {
    ($this:ident, $op:expr, $fun:ident) => {
        let disp = $op as i8 as i32;
        $this.$fun(disp);
    }
}

// PC relative 12 bits of displacement
macro_rules! d12_format {
    ($this:ident, $op:expr, $fun:ident) => {
        let d = ($op & 0x0fff) as i32;
        $this.$fun((d << 20) >> 20);
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

macro_rules! nm_nobus_format {
    ($this:ident, $op:expr, $fun:ident) => {
        let regs = ($op & 0x0ff0) >> 4;
        let rn = (regs >> 0x4) as usize;
        let rm = (regs & 0xf) as usize;
        $this.$fun(rm, rn);
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
    ($this:ident, $op:expr, $fun:ident) => {
        let rn = (($op & 0x0f00) >> 0x8) as usize;
        let i = $op as i8 as i32 as u32;
        $this.$fun(i, rn);
    }
}


macro_rules! do_op {
    ($this:ident, $bus:expr, $op:expr) => {
        match $op >> 12 {
            // we're starting with the most significant nibble
            0b0010 => {
                // least significant nibble
                match $op & 0xf {
                    0b0010 => { nm_format!($this, $bus, $op, mov_ls); },
                    0b0110 => { nm_format!($this, $bus, $op, mov_lm); },
                    0b1000 => { nm_nobus_format!($this, $op, tst); },
                    0b1001 => { nm_nobus_format!($this, $op, and); },
                    0b1010 => { nm_nobus_format!($this, $op, xor); },
                    0b1011 => { nm_nobus_format!($this, $op, or); },
                    _ => $this.op_least_significant_nibble_unknown($op)
                }
            },
            0b0011 => {
                match $op & 0xf {
                    0b0010 => { nm_nobus_format!($this, $op, cmp_hs); },
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
                    0b0001 => { nm_format!($this, $bus, $op, mov_wl); },
                    0b0010 => { nm_format!($this, $bus, $op, mov_ll); },
                    _ => $this.op_least_significant_nibble_unknown($op)
                }
            },
            0b0111 => { ni_format!($this, $op, add_i); },
            0b1000 => {
                match ($op & 0x0f00) >> 8 {
                    0b1011 => { d8_format!($this, $op, bf); },
                    _ => panic!("2nd nibble unknown: {:#06b} of op {:#06x}",
                                ($op & 0x0f00) >> 8, $op)
                }
            },
            0b1001 => { nd8_format!($this, $bus, $op, mov_wi); },
            0b1010 => { d12_format!($this, $op, bra); },
            0b1101 => { nd8_format!($this, $bus, $op, mov_li); },
            0b1110 => { ni_format!($this, $op, mov_i); },
            _ => $this.op_most_significant_nibble_unknown($op)
        }
    }
}
