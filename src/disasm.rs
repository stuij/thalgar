use bus::Bus;
use ops;
use sh2;

// macros for handily printing dissassembly fns

macro_rules! label {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, disp: i32) {
            println!("{} label (disp: {})", $name, disp);
        }
    }
}

// OP rm, rn
macro_rules! mn {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rm: usize, rn: usize) {
            println!("{} r{}, r{}", $name, rm, rn);
        }
    }
}

// OP @rm, rn
macro_rules! at_mn {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rm: usize, rn: usize) {
            println!("{} @r{}, r{}", $name, rm, rn);
        }
    }
}

// OP rm, @rn
macro_rules! m_at_n {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rm: usize, rn: usize) {
            println!("{} r{}, @r{}", $name, rm, rn);
        }
    }
}

// OP rm, @-rn
macro_rules! mn_post_dec {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rm: usize, rn: usize) {
            println!("{} r{}, @-r{}", $name, rm, rn);
        }
    }
}

// op @(disp, PC), rn
macro_rules! disp_n {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, d: u32, rn: usize) {
            println!("{} @({}, PC), r{}", $name, d, rn);
        }
    }
}

// OP imm, rn
macro_rules! imm_n {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, i: u32, rn: usize) {
            println!("{} {}, r{}", $name, i, rn);
        }
    }
}

// OP X, @-rn
macro_rules! n_post_dec {
    ($fun:ident, $name:expr, $src_reg:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rn: usize) {
            println!("{} {}, @-r{}", $name, $src_reg, rn);
        }
    }
}


pub struct Disassemble {
    pc: u32
}


impl Disassemble {
    pub fn new() -> Disassemble {
        Disassemble { pc: 0 }
    }

    pub fn disasemble<B: Bus>(&mut self, cpu: &sh2::Sh2, bus: &mut B) {
        self.pc = cpu.get_pc();
        self.print_addr(bus, cpu.get_pc());
    }

    pub fn print_addr<B: Bus>(&mut self, bus: &mut B, addr: u32) {
        let op = bus.read_word(addr);
        print!("{:#010x}   {:#06x}    ", addr, op);
        do_op!(self, bus, op);
    }

    pub fn print_range<B: Bus>(&mut self, bus: &mut B,
                               start: u32, end: u32) {
        for i in (start..end).filter(|x| x % 2 == 0) {
            self.pc = i;
            let val = bus.read_word(i);
            self.print_addr(bus, i);
        }
    }

    fn print_unknown_instr(&mut self, op: u16) {
        print!("unknown instruction: {:#06x}", op);
        println!();
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

    imm_n!(add_i, "add");
    mn!(cmp_hs, "cmp/hs");
    at_mn!(mov_ll, "mov.l");
    m_at_n!(mov_ls, "mov.l");
    imm_n!(mov_i, "mov");
    mn_post_dec!(mov_lm, "mov.l");
    n_post_dec!(sts_mpr, "sts.l", "pr");

    fn bf<B: Bus>(&mut self, bus: &mut B, disp: i32) {
        let addr = (self.pc + 4).wrapping_add((disp << 1) as u32);
        println!("bf label (addr: {:#010x}) (disp: {})", addr, disp);
    }

    fn bra<B: Bus>(&mut self, bus: &mut B, disp: i32) {
        let addr = (self.pc + 4).wrapping_add((disp << 1) as u32);
        println!("bra label (addr: {:#010x}) (disp: {})", addr, disp);
    }

    fn mov_li<B: Bus>(&mut self, bus: &mut B, disp: u32, rn: usize) {
        // PC = 4 bytes past current instr, with bottom 2 bits set to 0
        let pc = (self.pc + 4) & 0xfffffffc;
        let src = (disp << 2) + pc;
        println!("mov.l @({}, PC), r{} (addr: {:#010x})", disp, rn, src);
    }
}
