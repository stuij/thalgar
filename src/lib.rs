// the main cpu logic
// references to sections of the SH2 programming manual are enclosed
// in brackets. ex: [2.1]

struct Sh2 {
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
    pc: u32,
}


impl Sh2 {
    fn new() -> Sh2 {
        let sh2 = Sh2 {
            // TODO: r15/sp should be value of sp in the vector address table
            gpr: [ 0xDEADBEEF; 16],
            gbr: 0xDEADBEEF,
            sr_t: false, // undefined
            sr_s: false, // undefined
            sr_i: 0xF,
            sr_q: false, // undefined
            sr_m: false, // undefined
            vbr: 0x00000000,
            mach: 0xDEADBEEF,
            macl: 0xDEADBEEF,
            pr: 0xDEADBEEF,
            pc: 0x00000000, // TODO: Value of the pc in the vector address table
        };
        sh2
    }
}

// Register operands are always longwords (32 bits). When data in memory is
// loaded to a register and the memory operand is only a byte (8 bits) or a
// word (16 bits), it is sign-extended into a longword when stored into a
// register.


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
