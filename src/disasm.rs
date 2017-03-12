use bus::Bus;
use ops;
use sh2;

pub struct Disassemble;

impl Disassemble {
    pub fn disasemble<B: Bus>(&mut self, cpu: &sh2::Sh2, bus: &mut B) {
        let regs = cpu.get_regs();
        let op = bus.read_word(regs.pc);
        do_op!(self, bus, op);
    }

    fn movl<B: Bus>(&mut self, bus:&mut B, rm: u16, rn: u16) {
        println!("movl r{}, @-r{}", rm, rn);
    }
}
