use std::fmt;

use bus::Bus;


#[derive(Clone)]
pub struct Regs {
    // registers
    // [2.1] general purpose registers, sp is #15
    pub gpr: [u32; 16],

    // [2.2] control registers
    // The global base register functions as a base address for the indirect
    // GBR addressing mode to transfer data to the registers of on-chip
    // peripheral modules.
    pub gbr: u32,

    // The vector base register functions as the base address of the exception
    // processing vector area (including interrupts).
    pub vbr: u32,

    // the status register (sr) bits:
    pub sr_t: bool, // sr t bit (#0): various use
    pub sr_s: bool, // sr s (saturation) bit (#1): multiply/accumulate
    pub sr_i: u32, // interrupt request mask (#4-7): level 0-15
    pub sr_q: bool, // (#8) The Q and M bits are used by the DIV0U/S
    pub sr_m: bool, // (#9) and DIV1 instructions.
    // the rest of the bits are reserved


    // [2.3] system registers

    // Multiply and accumulate register high (MACH) Multiply and accumulate
    // register low (MACL): the registers for storing the results of multiply
    // and accumulate operations.
    pub mach: u32,
    pub macl: u32,

    // Procedure register (PR): store the return destination addresses for
    // subroutine procedures.
    pub pr: u32,

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
                write!(f, "  r{:#02}: {:#010x} ", offset, self.gpr[offset])
                    .unwrap();
            };
            write!(f, "\n").unwrap();
        };
        write!(f, "\n   pc: {:#010x}   vbr: {:#010x}   gbr: {:#010x}    \
                        pr: {:#010x}  mach: {:#010x}  macl: {:#010x} ",
               self.pc, self.vbr, self.gbr, self.pr, self.mach, self.macl)
            .unwrap();
        write!(f, "\n sr_t: {:<10}  sr_s: {:<10}  sr_q: {:<10}  \
                   sr_m: {:<10}  sr_i: {:#06b}",
               self.sr_t, self.sr_s, self.sr_q, self.sr_m, self.sr_i)
   }
}

// the main cpu logic
// references to sections of the SH2 programming manual are enclosed
// in brackets. ex: [2.1]
pub struct Sh2 {
    cycles: u64,
    regs: Regs,
    delay: bool,
    delay_pc: u32,
}

impl fmt::Display for Sh2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\
                   \n  cyc: {:#010x}   del: {:<8}  del_pc: {:#010x}",
               self.regs, self.cycles, self.delay, self.delay_pc)
    }
}


impl Sh2 {
    pub fn new() -> Sh2 {
        Sh2 {
            cycles: 0,
            regs: Regs::new(),
            delay: false,
            delay_pc: 0xdeadbeef,

        }
    }

    pub fn get_regs(&self) -> Regs {
        self.regs.clone()
    }

    pub fn get_pc(&self) -> u32 {
        self.regs.pc
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

        if self.delay {
            self.regs.pc = self.delay_pc;
            self.delay = false;
        } else {
            self.regs.pc += 2;
        }

        self.do_op(bus, op);
        self.cycles += 1;
    }

    fn op_most_significant_nibble_unknown(&mut self, op: u16) {
        panic!("\n\ndid not recognize most significant nibble \
                {:#06b} of op {:#06x}\n\nCPU state:\n{}\n\n",
               op >> 12, op, self)
    }


    fn op_least_significant_nibble_unknown(&mut self, op: u16) {
        panic!("\n\ndid not recognize least significant nibble \
                {:#06b} of op {:#06x} \n\nCPU state:\n{}\n\n",
               op & 0xF, op, self)
    }


    fn op_least_significant_byte_unknown(&mut self, op: u16) {
        panic!("\n\ndid not recognize least significant byte \
                {:#010b} of op {:#06x} \n\nCPU state:\n{}\n\n",
               op & 0xFF, op, self)
    }


    fn do_op<B: Bus>(&mut self, bus: &mut B, op: u16) {
        do_op!(self, bus, op);
    }

    // instruction handlers
    // doc in format:
    // instr        format            desc                            cyc  t-bit

    // 0010
    // MOV.L Rm, @Rn  0010nnnnmmmm0010  Rm → (Rn)                     1    -
    fn mov_ls<B: Bus>(&mut self, bus: &mut B, rm: usize, rn: usize) {
        bus.write_long(self.regs.gpr[rn], self.regs.gpr[rm]);
    }

    // MOV.L Rm,@–Rn  0010nnnnmmmm0110  Rn–4 → Rn, Rm → (Rn)          1    -
    fn mov_lm<B: Bus>(&mut self, bus: &mut B, rm: usize, rn: usize) {
        self.regs.gpr[rn] -= 4;
        bus.write_long(self.regs.gpr[rn], self.regs.gpr[rm]);
    }

    // TST Rm, Rn  0010nnnnmmmm1000  Rn & Rm → Rn  Rn & Rm; if the    1    test
    //                               result is 0, 1 → T                   result
    fn tst(&mut self, rm: usize, rn: usize) {
        let res = self.regs.gpr[rn] & self.regs.gpr[rm];
        self.regs.sr_t = if res == 0 { true } else { false };
    }

    // AND Rm, Rn  0010nnnnmmmm1001  Rn & Rm → Rn                     1    -
    fn and(&mut self, rm: usize, rn: usize) {
        self.regs.gpr[rn] &= self.regs.gpr[rm];
    }

    // XOR Rm, Rn  0010nnnnmmmm1010  Rn & Rm → Rn                     1    -
    fn xor(&mut self, rm: usize, rn: usize) {
        self.regs.gpr[rn] ^= self.regs.gpr[rm];
    }

    // OR Rm, Rn  0010nnnnmmmm1011  Rn | Rm → Rn                      1    -
    fn or(&mut self, rm: usize, rn: usize) {
        self.regs.gpr[rn] |= self.regs.gpr[rm];
    }


    // 0011
    // CMP/HS Rm, Rn  0011nnnnmmmm0010  If Rn≥Rm with                 1    Comp.
    //                                  unsigned data, 1 → T              result
    fn cmp_hs(&mut self, rm: usize, rn: usize) {
        if self.regs.gpr[rn] >= self.regs.gpr[rm] {
            self.regs.sr_t = true;
        } else {
            self.regs.sr_t = false;
        }
    }


    // 0100
    // STS.L PR,@–Rn  0100nnnn00100010  Rn–4→ Rn, PR → (Rn)           1    -
    fn sts_mpr<B: Bus>(&mut self, bus: &mut B, rn: usize) {
        self.regs.gpr[rn] -= 4;
        bus.write_long(self.regs.gpr[rn],
                       self.regs.pr);
        // TODO: no interrupts are allowed between this instr and the next.
        // Address errors are accepted.
    }


    // 0110
    // MOV.W @Rm,Rn  0110nnnnmmmm0001  (Rm) → Sign extension → Rn     1    -
    fn mov_wl<B: Bus>(&mut self, bus: &mut B, rm: usize, rn: usize) {
        self.regs.gpr[rn] =
            bus.read_word(self.regs.gpr[rm]) as i16 as i32 as u32;
    }

    // MOV.L @Rm, Rn  0110nnnnmmmm0010  (Rm) → Rn                     1    -
    fn mov_ll<B: Bus>(&mut self, bus: &mut B, rm: usize, rn: usize) {
        self.regs.gpr[rn] = bus.read_long(self.regs.gpr[rm]);
    }


    // 0111
    // ADD #imm, Rn  0111nnnniiiiiiii  Rn + imm → Rn                  1    -
    fn add_i(&mut self, imm: u32, rn: usize) {
        self.regs.gpr[rn] += imm;
    }


    // 1000
    // BF  label  10001011dddddddd  If T = 0, disp × 2 + PC → PC;     3/1  -
    //                              if T = 1, nop
    fn bf(&mut self, disp: i32) {
        if !self.regs.sr_t {
            self.regs.pc = (self.regs.pc + 2).wrapping_add((disp << 1) as u32);
            self.cycles += 2;
        }
    }

    // 1001
    // MOV.W @(disp:8,PC),Rn  1001nnnndddddddd  (disp × 2 + PC) →     1    -
    //                                           Sign extension → Rn
    fn mov_wi<B: Bus>(&mut self, bus: &mut B, disp: u32, rn: usize) {
        // PC = 4 bytes past current instr
        let src = (disp << 1) + self.regs.pc + 2;
        let val = bus.read_word(src) as i16 as i32 as u32;
        self.regs.gpr[rn] = val;
    }


    // 1010
    // BRA label  1010dddddddddddd  Delayed branch,                   2    -
    //                              disp × 2 + PC → PC
    fn bra(&mut self, disp: i32) {
        self.delay = true;
        self.delay_pc = (self.regs.pc + 2).wrapping_add((disp << 1) as u32);
        self.cycles += 1;
    }


    // 1101
    // MOV.L @(disp:8,PC),Rn  1101nnnndddddddd  (disp × 4 + PC) → Rn  1    -
    fn mov_li<B: Bus>(&mut self, bus: &mut B, disp: u32, rn: usize) {
        // PC = 4 bytes past current instr, with bottom 2 bits set to 0
        let pc = (self.regs.pc + 2) & 0xfffffffc;
        let src = (disp << 2) + pc;
        self.regs.gpr[rn] = bus.read_long(src);
    }


    // 1110
    // MOV #imm, Rn  1110nnnniiiiiiii  #imm → Sign extension → Rn     1    -
    fn mov_i(&mut self, imm: u32, rn: usize) {
        self.regs.gpr[rn] = imm;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestBus {
        addr: [u16; 2]
    }

    impl Bus for TestBus {
        #[allow(unused_variables)]
        fn read_word(&self, addr: u32) -> u16 {
            // just get the first word
            self.addr[0]
        }

        #[allow(unused_variables)]
        fn read_long(&self, addr: u32) -> u32 {
            (self.addr[0] as u32) << 16 |
            self.addr[1] as u32
        }

        #[allow(unused_variables)]
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
