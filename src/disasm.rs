use std::collections::HashMap;

use bus::Bus;

// macros for handily printing dissassembly fns
macro_rules! print_dis {
    ($this:ident, $fmt:expr) =>
        (if $this.print {print!(concat!($fmt, "\n"))});
    ($this:ident, $fmt:expr, $($arg:tt)*) =>
        (if $this.print { print!(concat!($fmt, "\n"), $($arg)*)});
}

macro_rules! label {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, disp: i32) {
            print_dis!(self, "{} label (disp: {:#x})", $name, disp);
        }
    }
}

// OP rm, rn
macro_rules! mn {
    ($fun:ident, $name:expr) => {
        fn $fun(&mut self, rm: usize, rn: usize) {
            print_dis!(self, "{} r{}, r{}", $name, rm, rn);
        }
    }
}

// OP @rm, rn
macro_rules! at_mn {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, rm: usize, rn: usize) {
            print_dis!(self, "{} @r{}, r{}", $name, rm, rn);
        }
    }
}

// OP rm, @rn
macro_rules! m_at_n {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, rm: usize, rn: usize) {
            print_dis!(self, "{} r{}, @r{}", $name, rm, rn);
        }
    }
}

// OP rm, @-rn
macro_rules! mn_post_dec {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, rm: usize, rn: usize) {
            print_dis!(self, "{} r{}, @-r{}", $name, rm, rn);
        }
    }
}

// op @(disp, PC), rn
macro_rules! disp_n {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, d: u32, rn: usize) {
            print_dis!(self, "{} @({:#x}, PC), r{}", $name, d, rn);
        }
    }
}

// OP imm, rn
macro_rules! imm_n {
    ($fun:ident, $name:expr) => {
        fn $fun(&mut self, i: u32, rn: usize) {
            print_dis!(self, "{} {:#x}, r{}", $name, i, rn);
        }
    }
}

// OP X, @-rn
macro_rules! n_post_dec {
    ($fun:ident, $name:expr, $src_reg:expr) => {
        fn $fun<B: Bus>(&mut self, _bus:&mut B, rn: usize) {
            print_dis!(self, "{} {}, @-r{}", $name, $src_reg, rn);
        }
    }
}


pub struct Disassemble {
    pc: u32,
    labels: HashMap<u32, String>,
    print: bool
}


impl Disassemble {
    pub fn new() -> Disassemble {
        Disassemble { pc: 0,
                      labels: HashMap::new(),
                      print: true,
        }
    }

    pub fn disasemble<B: Bus>(&mut self, bus: &mut B, pc: u32) {
        self.pc = pc;
        self.disass_addr(bus, pc);
    }

    pub fn disassemble_range<B: Bus>(&mut self, bus: &mut B,
                                     start: u32, end: u32, pc: u32) {
        self.pc = pc;
        self.print = false;
        self.print_range(bus, start, end);
        self.print = true;
        self.print_range(bus, start, end);
    }

    fn print_range<B: Bus>(&mut self, bus: &mut B,
                           start: u32, end: u32) {
        for i in (start..end).filter(|x| x % 2 == 0) {
            self.disass_addr(bus, i);
        }
    }

    fn disass_addr<B: Bus>(&mut self, bus: &mut B, addr: u32) {
        let op = bus.read_word(addr);
        let pre = if addr == self.pc { "->" } else { "" };
        let label = String::clone(self.labels.get(&addr)
                                       .unwrap_or(&String::from("")));

        if self.print {
            print!("{:<2} {:<5}  {:#010x}   {:#06x}    ",
                   pre, label, addr, op);
        }
        do_op!(self, bus, op);
    }

    fn print_unknown_instr(&mut self, op: u16) {
        print_dis!(self, "unknown instruction: {:#06x}", op);
    }

    fn add_label(&mut self, addr: u32) -> String {
        let label_name = format!("l-{}", self.labels.len());
        if !self.labels.contains_key(&addr) {
            self.labels.insert(addr, label_name);
        }
        String::clone(self.labels.get(&addr).unwrap())
    }

    fn op_most_significant_nibble_unknown(&mut self, op: u16) {
        self.print_unknown_instr(op);
    }

    fn op_least_significant_nibble_unknown(&mut self, op: u16) {
        self.print_unknown_instr(op);
    }

    fn op_least_significant_byte_unknown(&mut self, op: u16) {
        self.print_unknown_instr(op);
    }

    // compressed
    imm_n!(add_i, "add");
    mn!(cmp_hs, "cmp/hs");
    at_mn!(mov_ll, "mov.l");
    m_at_n!(mov_ls, "mov.l");
    imm_n!(mov_i, "mov");
    mn_post_dec!(mov_lm, "mov.l");
    n_post_dec!(sts_mpr, "sts.l", "pr");

    #[allow(unused_variables)]
    fn bf(&mut self, disp: i32) {
        let addr = (self.pc + 4).wrapping_add((disp << 1) as u32);
        let label = self.add_label(addr);
        print_dis!(self, "bf {}   (addr: {:#010x}, disp: {:#x})",
                   label, addr, disp);
    }

    #[allow(unused_variables)]
    fn bra(&mut self, disp: i32) {
        let addr = (self.pc + 4).wrapping_add((disp << 1) as u32);
        let label = self.add_label(addr);
        print_dis!(self, "bra {}   (addr: {:#010x}, disp: {:#x})",
                   label, addr, disp);
    }

    fn mov_li<B: Bus>(&mut self, bus: &mut B, disp: u32, rn: usize) {
        // PC = 4 bytes past current instr, with bottom 2 bits set to 0
        let pc = (self.pc + 4) & 0xfffffffc;
        let src = (disp << 2) + pc;
        let val = bus.read_long(src);
        print_dis!(self, "mov.l @({:#x}, PC), r{}   (addr: {:#010x}, val: {:#010x})",
                   disp, rn, src, val);
    }
}
