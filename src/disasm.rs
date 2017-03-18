use bus::Bus;
use ops;
use sh2;

pub struct Disassemble;

// macros for handily printing dissassembly fns

// OP rm, @-rn
macro_rules! nm_post_dec {
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



impl Disassemble {
    pub fn disasemble<B: Bus>(&mut self, cpu: &sh2::Sh2, bus: &mut B) {
        let regs = cpu.get_regs();
        let op = bus.read_word(regs.pc);
        print!("{:#010x}   {:#06x}    ", regs.pc, op);
        do_op!(self, bus, op);
    }

    fn op_most_significant_nibble_unknown(&mut self, op: u16) {
        print!("unknown instruction: {:#06x}", op);
    }

    fn op_least_significant_nibble_unknown(&mut self, op: u16) {
        print!("unknown instruction: {}", op);
    }

    fn op_least_significant_byte_unknown(&mut self, op: u16) {
        print!("unknown instruction: {}", op);
    }

    imm_n!(mov_i, "mov");
    disp_n!(movli, "mov.l");
    nm_post_dec!(movl, "mov.l");
    n_post_dec!(stsl_pr, "sts.l", "pr");
}
