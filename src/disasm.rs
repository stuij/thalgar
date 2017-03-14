use bus::Bus;
use ops;
use sh2;

pub struct Disassemble;

// macros for handily printing dissassembly fns

// OP rm, @-rn
macro_rules! nm_post_dec {
    ($fun:ident, $name:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rm: u16, rn: u16) {
            println!("{} r{}, @-r{}", $name, rm, rn);
        }
    }
}

macro_rules! n_post_dec {
    ($fun:ident, $name:expr, $src_reg:expr) => {
        fn $fun<B: Bus>(&mut self, bus:&mut B, rn: u16) {
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

    nm_post_dec!(movl, "mov.l");
    n_post_dec!(stsl_pr, "sts.l", "pr");
}
