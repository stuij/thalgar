use std::fmt;

use bus::Bus;
use disasm;
use ops;

#[derive(Clone)]
pub struct Regs {
    // registers
    // [2.1] general purpose registers, sp is #15
    gpr: [u32; 16],

    // [2.2] control registers
    // The global base register functions as a base address for the indirect
    // GBR addressing mode to transfer data to the registers of on-chip
    // peripheral modules.
    gbr: u32,

    // The vector base register functions as the base address of the exception
    // processing vector area (including interrupts).
    vbr: u32,

    // the status register (sr) bits:
    sr_t: bool, // sr t bit (#0): various use
    sr_s: bool, // sr s (saturation) bit (#1): multiply/accumulate
    sr_i: u32, // interrupt request mask (#4-7): level 0-15
    sr_q: bool, // (#8) The Q and M bits are used by the DIV0U/S
    sr_m: bool, // (#9) and DIV1 instructions.
    // the rest of the bits are reserved


    // [2.3] system registers

    // Multiply and accumulate register high (MACH) Multiply and accumulate
    // register low (MACL): the registers for storing the results of multiply
    // and accumulate operations.
    mach: u32,
    macl: u32,

    // Procedure register (PR): store the return destination addresses for
    // subroutine procedures.
    pr: u32,

    // program counter
    pub pc: u32,
}


// Register operands are always longwords (32 bits). When data in memory is
// loaded to a register and the memory operand is only a byte (8 bits) or a
// word (16 bits), it is sign-extended into a longword when stored into a
// register.
impl Regs {
    fn new() -> Regs {
        Regs {
            gpr: [ 0xDEADBEEF; 16],
            gbr: 0xDEADBEEF,
            sr_t: false, // undefined
            sr_s: true, // undefined
            sr_i: 0x00000000,
            sr_q: false, // undefined
            sr_m: false, // undefined
            vbr: 0x00000000,
            mach: 0xDEADBEEF,
            macl: 0xDEADBEEF,
            pr: 0xDEADBEEF,
            pc: 0xDEADBEEF
        }
    }

    // when we hard reset
    fn reset(&mut self, pc: u32, sp: u32) {
        self.vbr = 0x00000000;
        self.pc = pc;
        self.gpr[15] = sp;
        self.sr_i = 0xF;
    }
}

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..2 {
            for j in 0..8 {
                let offset = i*8 + j;
                write!(f, "  r{:#02}: {:#010x} ", offset, self.gpr[offset]);
            };
            println!()
        };
        write!(f, "\n   pc: {:#010x}   vbr: {:#010x}   gbr: {:#010x}    \
                        pr: {:#010x}  mach: {:#010x}  macl: {:#010x} ",
               self.pc, self.vbr, self.gbr, self.pr, self.mach, self.macl);
        write!(f, "\n sr_t: {:<10}  sr_s: {:<10}  sr_q: {:<10}  \
                   sr_m: {:<10}  sr_i: {:#06b}",
               self.sr_t, self.sr_s, self.sr_q, self.sr_m, self.sr_i)
   }
}

// the main cpu logic
// references to sections of the SH2 programming manual are enclosed
// in brackets. ex: [2.1]
pub struct Sh2 {
    regs: Regs,
    cycles: u64
}

impl fmt::Display for Sh2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.regs)
    }
}


impl Sh2 {
    pub fn new() -> Sh2 {
        Sh2 {
            regs: Regs::new(),
            cycles: 0
        }
    }

    pub fn get_regs(&self) -> Regs {
        self.regs.clone()
    }

    pub fn reset(&mut self, pc: u32, sp: u32) {
        self.regs.reset(pc, sp);
    }

    // This is not wholly kosher perhaps, but for the CPS3 we bypass
    // the bios code for now, as it depends on cdrom drivers. So we
    // set the vbr straight to the game code base (I think that is what is
    // happening).
    pub fn set_vbr(&mut self, vbr: u32) {
        self.regs.vbr = vbr;
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) {
        let op = bus.read_word(self.regs.pc);
        self.do_op(bus, op);
    }

    fn do_op<B: Bus>(&mut self, bus: &mut B, op: u16) {
        do_op!(self, bus, op);
    }

    // instruction handlers
    // doc in format:
    // instr           format            desc                   cyc  t-bit

    // MOV.L Rm,@–Rn   0010nnnnmmmm0110  Rn–4 → Rn, Rm → (Rn)   1    -
    fn movl<B: Bus>(&mut self, bus:&mut B, rm: u16, rn: u16) {
        self.regs.gpr[rn as usize] -= 4;
        bus.write_long(self.regs.gpr[rn as usize],
                       self.regs.gpr[rm as usize]);

        self.regs.pc += 2;
        self.cycles += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestBus {
        addr: [u16; 2]
    }

    impl Bus for TestBus {
        fn read_word(&self, addr: u32) -> u16 {
            // just get the first word
            self.addr[0]
        }

        fn write_long(&mut self, addr: u32, val: u32) {
            self.addr[0] = (val >> 16) as u16;
            self.addr[1] = (val & 0xFFFF) as u16;
        }
    }

    #[test]
    fn test_step_pc() {
        let mut bus = TestBus { addr: [0x2fd6, 0x2fe6] };
        let mut cpu = Sh2::new();
        cpu.reset(0x00000000, 0x1000000);
        cpu.step(&mut bus);
        assert_eq!(cpu.regs.pc, 0x00000002);
    }
}
