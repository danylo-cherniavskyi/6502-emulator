pub mod memory;

use memory::memory::{AddressingMode, Byte, Memory, MemoryLike, Word};

#[derive(PartialEq, Eq)]
pub enum Register {
    A,
    X,
    Y,
    SP,
    I,
    PS,
    None,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Instruction {
    // LDA
    LDA_IM,
    LDA_ZP,
    LDA_ZP_X,
    LDA_ABS,
    LDA_ABS_X,
    LDA_ABS_Y,
    LDA_IN_X,
    LDA_IN_Y,

    // LDX
    LDX_IM,
    LDX_ZP,
    LDX_ZP_Y,
    LDX_ABS,
    LDX_ABS_Y,

    // LDY
    LDY_IM,
    LDY_ZP,
    LDY_ZP_X,
    LDY_ABS,
    LDY_ABS_X,

    // STA
    STA_ZP,
    STA_ZP_X,
    STA_ABS,
    STA_ABS_X,
    STA_ABS_Y,
    STA_IN_X,
    STA_IN_Y,

    // STX
    STX_ZP,
    STX_ZP_Y,
    STX_ABS,

    // STY
    STY_ZP,
    STY_ZP_X,
    STY_ABS,

    // TRANSFER
    TAX,
    TAY,
    TXA,
    TYA,
    TSX,
    TXS,

    // STACK
    PHA,
    PHP,
    PLA,
    PLP,

    // LOGICAL
    // AND
    AND_IM,
    AND_ZP,
    AND_ZP_X,
    AND_ABS,
    AND_ABS_X,
    AND_ABS_Y,
    AND_IN_X,
    AND_IN_Y,
    // EXCLUSIVE OR
    EOR_IM,
    EOR_ZP,
    EOR_ZP_X,
    EOR_ABS,
    EOR_ABS_X,
    EOR_ABS_Y,
    EOR_IN_X,
    EOR_IN_Y,
    // OR
    ORA_IM,
    ORA_ZP,
    ORA_ZP_X,
    ORA_ABS,
    ORA_ABS_X,
    ORA_ABS_Y,
    ORA_IN_X,
    ORA_IN_Y,
    // BIT TEST
    BIT_ZP,
    BIT_ABS,
    // ARITHMETIC
    // ADD WITH CARRY
    ADC_IM,
    ADC_ZP,
    ADC_ZP_X,
    ADC_ABS,
    ADC_ABS_X,
    ADC_ABS_Y,
    ADC_IN_X,
    ADC_IN_Y,

    // SUBSTRACT WITH CARRY
    SBC_IM,
    SBC_ZP,
    SBC_ZP_X,
    SBC_ABS,
    SBC_ABS_X,
    SBC_ABS_Y,
    SBC_IN_X,
    SBC_IN_Y,

    // COMPARE
    CMP_IM,
    CMP_ZP,
    CMP_ZP_X,
    CMP_ABS,
    CMP_ABS_X,
    CMP_ABS_Y,
    CMP_IN_X,
    CMP_IN_Y,

    // COMPARE X
    CPX_IM,
    CPX_ZP,
    CPX_ABS,

    // COMPARE Y
    CPY_IM,
    CPY_ZP,
    CPY_ABS,

    // INCREMENTS
    INC_ZP,
    INC_ZP_X,
    INC_ABS,
    INC_ABS_X,

    // INCREMENT X
    INX,

    // INCREMENT Y
    INY,

    // DECREMENTS
    DEC_ZP,
    DEC_ZP_X,
    DEC_ABS,
    DEC_ABS_X,

    // DECREMENT X
    DEX,

    // DECREMENT Y
    DEY,

    // SHIFTS
    // ARITHMETIC SHIFT LEFT
    ASL_A,
    ASL_ZP,
    ASL_ZP_X,
    ASL_ABS,
    ASL_ABS_X,

    // LOGICAL SHIFT RIGHT
    LSR_A,
    LSR_ZP,
    LSR_ZP_X,
    LSR_ABS,
    LSR_ABS_X,

    // ROTATE LEFT
    ROL_A,
    ROL_ZP,
    ROL_ZP_X,
    ROL_ABS,
    ROL_ABS_X,

    // ROTATE RIGHT
    ROR_A,
    ROR_ZP,
    ROR_ZP_X,
    ROR_ABS,
    ROR_ABS_X,

    // JUMPS AND CALLS
    JMP_ABS,
    JMP_IN,
    JSR_ABS,
    RTS_IM,

    // BRANCHES
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    INVALID,
}

impl From<u8> for Instruction {
    fn from(a: u8) -> Self {
        match a {
            // LDA
            0xA9 => Instruction::LDA_IM,
            0xA5 => Instruction::LDA_ZP,
            0xB5 => Instruction::LDA_ZP_X,
            0xAD => Instruction::LDA_ABS,
            0xBD => Instruction::LDA_ABS_X,
            0xB9 => Instruction::LDA_ABS_Y,
            0xA1 => Instruction::LDA_IN_X,
            0xB1 => Instruction::LDA_IN_Y,
            // LDX
            0xA2 => Instruction::LDX_IM,
            0xA6 => Instruction::LDX_ZP,
            0xB6 => Instruction::LDX_ZP_Y,
            0xAE => Instruction::LDX_ABS,
            0xBE => Instruction::LDX_ABS_Y,
            // LDY
            0xA0 => Instruction::LDY_IM,
            0xA4 => Instruction::LDY_ZP,
            0xB4 => Instruction::LDY_ZP_X,
            0xAC => Instruction::LDY_ABS,
            0xBC => Instruction::LDY_ABS_X,
            // STA
            0x85 => Instruction::STA_ZP,
            0x95 => Instruction::STA_ZP_X,
            0x8D => Instruction::STA_ABS,
            0x9D => Instruction::STA_ABS_X,
            0x99 => Instruction::STA_ABS_Y,
            0x81 => Instruction::STA_IN_X,
            0x91 => Instruction::STA_IN_Y,
            // STX
            0x86 => Instruction::STX_ZP,
            0x96 => Instruction::STX_ZP_Y,
            0x8E => Instruction::STX_ABS,
            // STY
            0x84 => Instruction::STY_ZP,
            0x94 => Instruction::STY_ZP_X,
            0x8C => Instruction::STY_ABS,
            // Transfer
            0xAA => Instruction::TAX,
            0xA8 => Instruction::TAY,
            0x8A => Instruction::TXA,
            0x98 => Instruction::TYA,
            0xBA => Instruction::TSX,
            0x9A => Instruction::TXS,
            // Stack
            0x48 => Instruction::PHA,
            0x08 => Instruction::PHP,
            0x68 => Instruction::PLA,
            0x28 => Instruction::PLP,
            // Logical
            // And
            0x29 => Instruction::AND_IM,
            0x25 => Instruction::AND_ZP,
            0x35 => Instruction::AND_ZP_X,
            0x2D => Instruction::AND_ABS,
            0x3D => Instruction::AND_ABS_X,
            0x39 => Instruction::AND_ABS_Y,
            0x21 => Instruction::AND_IN_X,
            0x31 => Instruction::AND_IN_Y,
            // Exclusive Or
            0x49 => Instruction::EOR_IM,
            0x45 => Instruction::EOR_ZP,
            0x55 => Instruction::EOR_ZP_X,
            0x4D => Instruction::EOR_ABS,
            0x5D => Instruction::EOR_ABS_X,
            0x59 => Instruction::EOR_ABS_Y,
            0x41 => Instruction::EOR_IN_X,
            0x51 => Instruction::EOR_IN_Y,
            // Or
            0x09 => Instruction::ORA_IM,
            0x05 => Instruction::ORA_ZP,
            0x15 => Instruction::ORA_ZP_X,
            0x0D => Instruction::ORA_ABS,
            0x1D => Instruction::ORA_ABS_X,
            0x19 => Instruction::ORA_ABS_Y,
            0x01 => Instruction::ORA_IN_X,
            0x11 => Instruction::ORA_IN_Y,
            // Bit
            0x24 => Instruction::BIT_ZP,
            0x2C => Instruction::BIT_ABS,
            // Arithmetic
            // ADC
            0x69 => Instruction::ADC_IM,
            0x65 => Instruction::ADC_ZP,
            0x75 => Instruction::ADC_ZP_X,
            0x6D => Instruction::ADC_ABS,
            0x7D => Instruction::ADC_ABS_X,
            0x79 => Instruction::ADC_ABS_Y,
            0x61 => Instruction::ADC_IN_X,
            0x71 => Instruction::ADC_IN_Y,
            // SBC
            0xE9 => Instruction::SBC_IM,
            0xE5 => Instruction::SBC_ZP,
            0xF5 => Instruction::SBC_ZP_X,
            0xED => Instruction::SBC_ABS,
            0xFD => Instruction::SBC_ABS_X,
            0xF9 => Instruction::SBC_ABS_Y,
            0xE1 => Instruction::SBC_IN_X,
            0xF1 => Instruction::SBC_IN_Y,
            // CMP
            0xC9 => Instruction::CMP_IM,
            0xC5 => Instruction::CMP_ZP,
            0xD5 => Instruction::CMP_ZP_X,
            0xCD => Instruction::CMP_ABS,
            0xDD => Instruction::CMP_ABS_X,
            0xD9 => Instruction::CMP_ABS_Y,
            0xC1 => Instruction::CMP_IN_X,
            0xD1 => Instruction::CMP_IN_Y,
            // CPX
            0xE0 => Instruction::CPX_IM,
            0xE4 => Instruction::CPX_ZP,
            0xEC => Instruction::CPX_ABS,
            // CPY
            0xC0 => Instruction::CPY_IM,
            0xC4 => Instruction::CPY_ZP,
            0xCC => Instruction::CPY_ABS,
            // Increments
            // INC
            0xE6 => Instruction::INC_ZP,
            0xF6 => Instruction::INC_ZP_X,
            0xEE => Instruction::INC_ABS,
            0xFE => Instruction::INC_ABS_X,
            // INX
            0xE8 => Instruction::INX,
            // INY
            0xC8 => Instruction::INY,
            // Decrements
            // DEC
            0xC6 => Instruction::DEC_ZP,
            0xD6 => Instruction::DEC_ZP_X,
            0xCE => Instruction::DEC_ABS,
            0xDE => Instruction::DEC_ABS_X,
            // DEX
            0xCA => Instruction::DEX,
            // DEY
            0x88 => Instruction::DEY,

            // Shifts
            // ASL
            0x0A => Instruction::ASL_A,
            0x06 => Instruction::ASL_ZP,
            0x16 => Instruction::ASL_ZP_X,
            0x0E => Instruction::ASL_ABS,
            0x1E => Instruction::ASL_ABS_X,

            // LSR
            0x4A => Instruction::LSR_A,
            0x46 => Instruction::LSR_ZP,
            0x56 => Instruction::LSR_ZP_X,
            0x4E => Instruction::LSR_ABS,
            0x5E => Instruction::LSR_ABS_X,

            // ROL
            0x2A => Instruction::ROL_A,
            0x26 => Instruction::ROL_ZP,
            0x36 => Instruction::ROL_ZP_X,
            0x2E => Instruction::ROL_ABS,
            0x3E => Instruction::ROL_ABS_X,

            // ROR
            0x6A => Instruction::ROR_A,
            0x66 => Instruction::ROR_ZP,
            0x76 => Instruction::ROR_ZP_X,
            0x6E => Instruction::ROR_ABS,
            0x7E => Instruction::ROR_ABS_X,

            // Jumps and Calls
            0x4C => Instruction::JMP_ABS,
            0x6C => Instruction::JMP_IN,
            0x20 => Instruction::JSR_ABS,
            0x60 => Instruction::RTS_IM,
            // Branches
            0x90 => Instruction::BCC,
            0xB0 => Instruction::BCS,
            0xF0 => Instruction::BEQ,
            0x30 => Instruction::BMI,
            0xD0 => Instruction::BNE,
            0x10 => Instruction::BPL,
            0x50 => Instruction::BVC,
            0x70 => Instruction::BVS,

            _ => Instruction::INVALID,
        }
    }
}

impl From<Instruction> for u8 {
    fn from(a: Instruction) -> Self {
        match a {
            // LDA
            Instruction::LDA_IM => 0xA9,
            Instruction::LDA_ZP => 0xA5,
            Instruction::LDA_ZP_X => 0xB5,
            Instruction::LDA_ABS => 0xAD,
            Instruction::LDA_ABS_X => 0xBD,
            Instruction::LDA_ABS_Y => 0xB9,
            Instruction::LDA_IN_X => 0xA1,
            Instruction::LDA_IN_Y => 0xB1,
            // LDX
            Instruction::LDX_IM => 0xA2,
            Instruction::LDX_ZP => 0xA6,
            Instruction::LDX_ZP_Y => 0xB6,
            Instruction::LDX_ABS => 0xAE,
            Instruction::LDX_ABS_Y => 0xBE,
            // LDY
            Instruction::LDY_IM => 0xA0,
            Instruction::LDY_ZP => 0xA4,
            Instruction::LDY_ZP_X => 0xB4,
            Instruction::LDY_ABS => 0xAC,
            Instruction::LDY_ABS_X => 0xBC,

            // STA
            Instruction::STA_ZP => 0x85,
            Instruction::STA_ZP_X => 0x95,
            Instruction::STA_ABS => 0x8D,
            Instruction::STA_ABS_X => 0x9D,
            Instruction::STA_ABS_Y => 0x99,
            Instruction::STA_IN_X => 0x81,
            Instruction::STA_IN_Y => 0x91,
            // STX
            Instruction::STX_ZP => 0x86,
            Instruction::STX_ZP_Y => 0x96,
            Instruction::STX_ABS => 0x8E,
            // STY
            Instruction::STY_ZP => 0x84,
            Instruction::STY_ZP_X => 0x94,
            Instruction::STY_ABS => 0x8C,
            // Transfer
            Instruction::TAX => 0xAA,
            Instruction::TAY => 0xA8,
            Instruction::TXA => 0x8A,
            Instruction::TYA => 0x98,
            Instruction::TSX => 0xBA,
            Instruction::TXS => 0x9A,
            // Stack
            Instruction::PHA => 0x48,
            Instruction::PHP => 0x08,
            Instruction::PLA => 0x68,
            Instruction::PLP => 0x28,
            // Logical
            // And
            Instruction::AND_IM => 0x29,
            Instruction::AND_ZP => 0x25,
            Instruction::AND_ZP_X => 0x35,
            Instruction::AND_ABS => 0x2D,
            Instruction::AND_ABS_X => 0x3D,
            Instruction::AND_ABS_Y => 0x39,
            Instruction::AND_IN_X => 0x21,
            Instruction::AND_IN_Y => 0x31,
            // Exclusive Or
            Instruction::EOR_IM => 0x49,
            Instruction::EOR_ZP => 0x45,
            Instruction::EOR_ZP_X => 0x55,
            Instruction::EOR_ABS => 0x4D,
            Instruction::EOR_ABS_X => 0x5D,
            Instruction::EOR_ABS_Y => 0x59,
            Instruction::EOR_IN_X => 0x41,
            Instruction::EOR_IN_Y => 0x51,
            // Or
            Instruction::ORA_IM => 0x09,
            Instruction::ORA_ZP => 0x05,
            Instruction::ORA_ZP_X => 0x15,
            Instruction::ORA_ABS => 0x0D,
            Instruction::ORA_ABS_X => 0x1D,
            Instruction::ORA_ABS_Y => 0x19,
            Instruction::ORA_IN_X => 0x01,
            Instruction::ORA_IN_Y => 0x11,
            // Bit
            Instruction::BIT_ZP => 0x24,
            Instruction::BIT_ABS => 0x2C,
            // Arithmetic
            // ADC
            Instruction::ADC_IM => 0x69,
            Instruction::ADC_ZP => 0x65,
            Instruction::ADC_ZP_X => 0x75,
            Instruction::ADC_ABS => 0x6D,
            Instruction::ADC_ABS_X => 0x7D,
            Instruction::ADC_ABS_Y => 0x79,
            Instruction::ADC_IN_X => 0x61,
            Instruction::ADC_IN_Y => 0x71,
            // SBC
            Instruction::SBC_IM => 0xE9,
            Instruction::SBC_ZP => 0xE5,
            Instruction::SBC_ZP_X => 0xF5,
            Instruction::SBC_ABS => 0xED,
            Instruction::SBC_ABS_X => 0xFD,
            Instruction::SBC_ABS_Y => 0xF9,
            Instruction::SBC_IN_X => 0xE1,
            Instruction::SBC_IN_Y => 0xF1,
            // CMP
            Instruction::CMP_IM => 0xC9,
            Instruction::CMP_ZP => 0xC5,
            Instruction::CMP_ZP_X => 0xD5,
            Instruction::CMP_ABS => 0xCD,
            Instruction::CMP_ABS_X => 0xDD,
            Instruction::CMP_ABS_Y => 0xD9,
            Instruction::CMP_IN_X => 0xC1,
            Instruction::CMP_IN_Y => 0xD1,
            // CPX
            Instruction::CPX_IM => 0xE0,
            Instruction::CPX_ZP => 0xE4,
            Instruction::CPX_ABS => 0xEC,
            // CPY
            Instruction::CPY_IM => 0xC0,
            Instruction::CPY_ZP => 0xC4,
            Instruction::CPY_ABS => 0xCC,
            // Increments
            // INC
            Instruction::INC_ZP => 0xE6,
            Instruction::INC_ZP_X => 0xF6,
            Instruction::INC_ABS => 0xEE,
            Instruction::INC_ABS_X => 0xFE,
            // INX
            Instruction::INX => 0xE8,
            // INY
            Instruction::INY => 0xC8,
            // Decrements
            // DEC
            Instruction::DEC_ZP => 0xC6,
            Instruction::DEC_ZP_X => 0xD6,
            Instruction::DEC_ABS => 0xCE,
            Instruction::DEC_ABS_X => 0xDE,
            // DEX
            Instruction::DEX => 0xCA,
            // DEY
            Instruction::DEY => 0x88,
            // Shifts
            // ASL
            Instruction::ASL_A => 0x0A,
            Instruction::ASL_ZP => 0x06,
            Instruction::ASL_ZP_X => 0x16,
            Instruction::ASL_ABS => 0x0E,
            Instruction::ASL_ABS_X => 0x1E,
            // LSR
            Instruction::LSR_A => 0x4A,
            Instruction::LSR_ZP => 0x46,
            Instruction::LSR_ZP_X => 0x56,
            Instruction::LSR_ABS => 0x4E,
            Instruction::LSR_ABS_X => 0x5E,
            // ROL
            Instruction::ROL_A => 0x2A,
            Instruction::ROL_ZP => 0x26,
            Instruction::ROL_ZP_X => 0x36,
            Instruction::ROL_ABS => 0x2E,
            Instruction::ROL_ABS_X => 0x3E,
            // ROR
            Instruction::ROR_A => 0x6A,
            Instruction::ROR_ZP => 0x66,
            Instruction::ROR_ZP_X => 0x76,
            Instruction::ROR_ABS => 0x6E,
            Instruction::ROR_ABS_X => 0x7E,
            // Jumps and Calls
            Instruction::JMP_ABS => 0x4C,
            Instruction::JMP_IN => 0x6C,
            Instruction::JSR_ABS => 0x20,
            Instruction::RTS_IM => 0x60,
            // Branches
            Instruction::BCC => 0x90,
            Instruction::BCS => 0xB0,
            Instruction::BEQ => 0xF0,
            Instruction::BMI => 0x30,
            Instruction::BNE => 0xD0,
            Instruction::BPL => 0x10,
            Instruction::BVC => 0x50,
            Instruction::BVS => 0x70,

            Instruction::INVALID => 0xFF,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    cycles: u64,
    pc: Word,
    sp: Byte,
    a: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            cycles: 0,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
        }
    }
}

impl CPU {
    pub fn reset(&mut self) {
        self.cycles = 0;
        self.pc = 0;
        self.sp = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = 0;
    }

    pub fn get_status(&self) -> Byte {
        return self.status;
    }

    pub fn get_carry(&self) -> bool {
        return (self.status & (1 << 0)) != 0;
    }

    pub fn set_carry(&mut self, value: bool) {
        let value_bin = (value as u8) << 0;
        self.status = (self.status & !(1 << 0)) | value_bin;
    }

    pub fn get_zero(&self) -> bool {
        return (self.status & (1 << 1)) != 0;
    }

    pub fn set_zero(&mut self, value: bool) {
        let value_bin = (value as u8) << 1;
        self.status = (self.status & !(1 << 1)) | value_bin;
    }

    pub fn get_interrupt_disable(&self) -> bool {
        return (self.status & (1 << 2)) != 0;
    }

    pub fn set_interrupt_disable(&mut self, value: bool) {
        let value_bin = (value as u8) << 2;
        self.status = (self.status & !(1 << 2)) | value_bin;
    }

    pub fn get_decimal_mode(&self) -> bool {
        return (self.status & (1 << 3)) != 0;
    }

    pub fn set_decimal_mode(&mut self, value: bool) {
        let value_bin = (value as u8) << 3;
        self.status = (self.status & !(1 << 3)) | value_bin;
    }

    pub fn get_break_command(&self) -> bool {
        return (self.status & (1 << 4)) != 0;
    }

    pub fn set_break_command(&mut self, value: bool) {
        let value_bin = (value as u8) << 4;
        self.status = (self.status & !(1 << 4)) | value_bin;
    }

    pub fn get_overflow(&self) -> bool {
        return (self.status & (1 << 5)) != 0;
    }

    pub fn set_overflow(&mut self, value: bool) {
        let value_bin = (value as u8) << 5;
        self.status = (self.status & !(1 << 5)) | value_bin;
    }

    pub fn get_negative(&self) -> bool {
        return (self.status & (1 << 6)) != 0;
    }

    pub fn set_negative(&mut self, value: bool) {
        let value_bin = (value as u8) << 6;
        self.status = (self.status & !(1 << 6)) | value_bin;
    }

    // Sets flags based on number passed
    fn test_number(&mut self, num: u8) {
        self.set_zero(num == 0);
        self.set_negative((num & 0b1000_0000) != 0);
    }

    pub fn execute(&mut self, memory: &mut Memory, i: Instruction) {
        match i {
            // LDA
            Instruction::LDA_IM => self.lda_immediate(memory),
            Instruction::LDA_ZP => self.lda_zero_page(memory),
            Instruction::LDA_ZP_X => self.lda_zero_page_x(memory),
            Instruction::LDA_ABS => self.lda_absolute(memory),
            Instruction::LDA_ABS_X => self.lda_absolute_x(memory),
            Instruction::LDA_ABS_Y => self.lda_absolute_y(memory),
            Instruction::LDA_IN_X => self.lda_indirect_x(memory),
            Instruction::LDA_IN_Y => self.lda_indirect_y(memory),
            // LDX
            Instruction::LDX_IM => self.ldx_immediate(memory),
            Instruction::LDX_ZP => self.ldx_zero_page(memory),
            Instruction::LDX_ZP_Y => self.ldx_zero_page_y(memory),
            Instruction::LDX_ABS => self.ldx_absolute(memory),
            Instruction::LDX_ABS_Y => self.ldx_absolute_y(memory),
            // LDY
            Instruction::LDY_IM => self.ldy_immediate(memory),
            Instruction::LDY_ZP => self.ldy_zero_page(memory),
            Instruction::LDY_ZP_X => self.ldy_zero_page_x(memory),
            Instruction::LDY_ABS => self.ldy_absolute(memory),
            Instruction::LDY_ABS_X => self.ldy_absolute_x(memory),

            // STA
            Instruction::STA_ZP => self.sta_zero_page(memory),
            Instruction::STA_ZP_X => self.sta_zero_page_x(memory),
            Instruction::STA_ABS => self.sta_absolute(memory),
            Instruction::STA_ABS_X => self.sta_absolute_x(memory),
            Instruction::STA_ABS_Y => self.sta_absolute_y(memory),
            Instruction::STA_IN_X => self.sta_indirect_x(memory),
            Instruction::STA_IN_Y => self.sta_indirect_y(memory),
            // STX
            Instruction::STX_ZP => self.stx_zero_page(memory),
            Instruction::STX_ZP_Y => self.stx_zero_page_y(memory),
            Instruction::STX_ABS => self.stx_absolute(memory),
            // STY
            Instruction::STY_ZP => self.sty_zero_page(memory),
            Instruction::STY_ZP_X => self.sty_zero_page_x(memory),
            Instruction::STY_ABS => self.sty_absolute(memory),
            // Transfer
            Instruction::TAX => self.transfer_a_x(),
            Instruction::TAY => self.transfer_a_y(),
            Instruction::TXA => self.transfer_x_a(),
            Instruction::TYA => self.transfer_y_a(),
            Instruction::TSX => self.transfer_s_x(),
            Instruction::TXS => self.transfer_x_s(),
            // Stack
            Instruction::PHA => self.push_accumulator(memory),
            Instruction::PHP => self.push_processor_status(memory),
            Instruction::PLA => self.pull_accumulator(memory),
            Instruction::PLP => self.pull_processor_status(memory),
            // Logical
            // And
            Instruction::AND_IM => self.and_immediate(memory),
            Instruction::AND_ZP => self.and_zero_page(memory),
            Instruction::AND_ZP_X => self.and_zero_page_x(memory),
            Instruction::AND_ABS => self.and_absolute(memory),
            Instruction::AND_ABS_X => self.and_absolute_x(memory),
            Instruction::AND_ABS_Y => self.and_absolute_y(memory),
            Instruction::AND_IN_X => self.and_indirect_x(memory),
            Instruction::AND_IN_Y => self.and_indirect_y(memory),
            // Exclusive Or
            Instruction::EOR_IM => self.eor_immediate(memory),
            Instruction::EOR_ZP => self.eor_zero_page(memory),
            Instruction::EOR_ZP_X => self.eor_zero_page_x(memory),
            Instruction::EOR_ABS => self.eor_absolute(memory),
            Instruction::EOR_ABS_X => self.eor_absolute_x(memory),
            Instruction::EOR_ABS_Y => self.eor_absolute_y(memory),
            Instruction::EOR_IN_X => self.eor_indirect_x(memory),
            Instruction::EOR_IN_Y => self.eor_indirect_y(memory),
            // Or
            Instruction::ORA_IM => self.ora_immediate(memory),
            Instruction::ORA_ZP => self.ora_zero_page(memory),
            Instruction::ORA_ZP_X => self.ora_zero_page_x(memory),
            Instruction::ORA_ABS => self.ora_absolute(memory),
            Instruction::ORA_ABS_X => self.ora_absolute_x(memory),
            Instruction::ORA_ABS_Y => self.ora_absolute_y(memory),
            Instruction::ORA_IN_X => self.ora_indirect_x(memory),
            Instruction::ORA_IN_Y => self.ora_indirect_y(memory),
            // Bit
            Instruction::BIT_ZP => self.bit_zero_page(memory),
            Instruction::BIT_ABS => self.bit_absolute(memory),

            // Arithmetic
            // ADC
            Instruction::ADC_IM => self.adc_immediate(memory),
            Instruction::ADC_ZP => self.adc_zero_page(memory),
            Instruction::ADC_ZP_X => self.adc_zero_page_x(memory),
            Instruction::ADC_ABS => self.adc_absolute(memory),
            Instruction::ADC_ABS_X => self.adc_absolute_x(memory),
            Instruction::ADC_ABS_Y => self.adc_absolute_y(memory),
            Instruction::ADC_IN_X => self.adc_indirect_x(memory),
            Instruction::ADC_IN_Y => self.adc_indirect_y(memory),
            // SBC
            Instruction::SBC_IM => self.sbc_immediate(memory),
            Instruction::SBC_ZP => self.sbc_zero_page(memory),
            Instruction::SBC_ZP_X => self.sbc_zero_page_x(memory),
            Instruction::SBC_ABS => self.sbc_absolute(memory),
            Instruction::SBC_ABS_X => self.sbc_absolute_x(memory),
            Instruction::SBC_ABS_Y => self.sbc_absolute_y(memory),
            Instruction::SBC_IN_X => self.sbc_indirect_x(memory),
            Instruction::SBC_IN_Y => self.sbc_indirect_y(memory),
            // CMP
            Instruction::CMP_IM => self.cmp_immediate(memory),
            Instruction::CMP_ZP => self.cmp_zero_page(memory),
            Instruction::CMP_ZP_X => self.cmp_zero_page_x(memory),
            Instruction::CMP_ABS => self.cmp_absolute(memory),
            Instruction::CMP_ABS_X => self.cmp_absolute_x(memory),
            Instruction::CMP_ABS_Y => self.cmp_absolute_y(memory),
            Instruction::CMP_IN_X => self.cmp_indirect_x(memory),
            Instruction::CMP_IN_Y => self.cmp_indirect_y(memory),
            // CPX
            Instruction::CPX_IM => self.cpx_immediate(memory),
            Instruction::CPX_ZP => self.cpx_zero_page(memory),
            Instruction::CPX_ABS => self.cpx_absolute(memory),
            // CPY
            Instruction::CPY_IM => self.cpy_immediate(memory),
            Instruction::CPY_ZP => self.cpy_zero_page(memory),
            Instruction::CPY_ABS => self.cpy_absolute(memory),
            // Increments
            // INC
            Instruction::INC_ZP => self.inc_zero_page(memory),
            Instruction::INC_ZP_X => self.inc_zero_page_x(memory),
            Instruction::INC_ABS => self.inc_absolute(memory),
            Instruction::INC_ABS_X => self.inc_absolute_x(memory),
            // INX
            Instruction::INX => self.inx(memory),
            // INY
            Instruction::INY => self.iny(memory),
            // Decrements
            // DEC
            Instruction::DEC_ZP => self.dec_zero_page(memory),
            Instruction::DEC_ZP_X => self.dec_zero_page_x(memory),
            Instruction::DEC_ABS => self.dec_absolute(memory),
            Instruction::DEC_ABS_X => self.dec_absolute_x(memory),
            // DEX
            Instruction::DEX => self.dex(memory),
            // DEY
            Instruction::DEY => self.dey(memory),
            // Shifts
            // ASL
            Instruction::ASL_A => self.asl_accumulator(memory),
            Instruction::ASL_ZP => self.asl_zero_page(memory),
            Instruction::ASL_ZP_X => self.asl_zero_page_x(memory),
            Instruction::ASL_ABS => self.asl_absolute(memory),
            Instruction::ASL_ABS_X => self.asl_absolute_x(memory),
            // LSR
            Instruction::LSR_A => self.lsr_accumulator(memory),
            Instruction::LSR_ZP => self.lsr_zero_page(memory),
            Instruction::LSR_ZP_X => self.lsr_zero_page_x(memory),
            Instruction::LSR_ABS => self.lsr_absolute(memory),
            Instruction::LSR_ABS_X => self.lsr_absolute_x(memory),
            // ROL
            Instruction::ROL_A => self.rol_accumulator(memory),
            Instruction::ROL_ZP => self.rol_zero_page(memory),
            Instruction::ROL_ZP_X => self.rol_zero_page_x(memory),
            Instruction::ROL_ABS => self.rol_absolute(memory),
            Instruction::ROL_ABS_X => self.rol_absolute_x(memory),
            // ROR
            Instruction::ROR_A => self.ror_accumulator(memory),
            Instruction::ROR_ZP => self.ror_zero_page(memory),
            Instruction::ROR_ZP_X => self.ror_zero_page_x(memory),
            Instruction::ROR_ABS => self.ror_absolute(memory),
            Instruction::ROR_ABS_X => self.ror_absolute_x(memory),
            // Jumps and Calls
            Instruction::JMP_ABS => self.jmp_absolute(memory),
            Instruction::JMP_IN => self.jmp_indirect(memory),
            Instruction::JSR_ABS => self.jsr_absolute(memory),
            Instruction::RTS_IM => self.rts_implied(memory),
            // Branches
            Instruction::BCC => self.bcc(memory),
            Instruction::BCS => self.bcs(memory),
            Instruction::BEQ => self.beq(memory),
            Instruction::BMI => self.bmi(memory),
            Instruction::BNE => self.bne(memory),
            Instruction::BPL => self.bpl(memory),
            Instruction::BVC => self.bvc(memory),
            Instruction::BVS => self.bvs(memory),

            Instruction::INVALID => println!("Error: Invalid instruction"),
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> Instruction {
        let instruction: Byte = memory.read(self.pc);
        self.pc += 1;

        return Instruction::from(instruction);
    }
}

macro_rules! ld {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident, $addr_mode: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Immediate, 2);
            cycles.insert(AddressingMode::ZeroPage, 3);
            cycles.insert(AddressingMode::ZeroPageReg, 4);
            cycles.insert(AddressingMode::Absolute, 4);
            cycles.insert(AddressingMode::AbsoluteReg, 4);
            cycles.insert(AddressingMode::IndirectX, 6);
            cycles.insert(AddressingMode::IndirectY, 5);

            let mut page_crossed: bool = false;

            let value = match $addr_mode {
                AddressingMode::Immediate => memory.read_immediate(&mut self.pc),
                AddressingMode::ZeroPage => memory.read_zero_page(&mut self.pc),
                AddressingMode::ZeroPageReg => {
                    memory.read_zero_page_x(&mut self.pc, self.$addr_reg)
                }
                AddressingMode::Absolute => memory.read_absolute(&mut self.pc),
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(
                    &mut self.pc,
                    self.$addr_reg,
                    &mut page_crossed,
                ),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => {
                    memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed)
                }
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            self.$reg_name = value;
            self.test_number(value);

            self.cycles += cycles[$addr_mode];
            self.cycles += if page_crossed { 1 } else { 0 }
        }
    };
}

impl CPU {
    ld! {lda_immediate, a, x, &AddressingMode::Immediate}
    ld! {lda_zero_page, a, x, &AddressingMode::ZeroPage}
    ld! {lda_zero_page_x, a, x, &AddressingMode::ZeroPageReg}
    ld! {lda_absolute, a, x, &AddressingMode::Absolute}
    ld! {lda_absolute_x, a, x, &AddressingMode::AbsoluteReg}
    ld! {lda_absolute_y, a, y, &AddressingMode::AbsoluteReg}
    ld! {lda_indirect_x, a, x, &AddressingMode::IndirectX}
    ld! {lda_indirect_y, a, y, &AddressingMode::IndirectY}
}

impl CPU {
    ld! {ldx_immediate, x, y, &AddressingMode::Immediate}
    ld! {ldx_zero_page, x, y, &AddressingMode::ZeroPage}
    ld! {ldx_zero_page_y, x, y, &AddressingMode::ZeroPageReg}
    ld! {ldx_absolute, x, y, &AddressingMode::Absolute}
    ld! {ldx_absolute_y, x, y, &AddressingMode::AbsoluteReg}
}

impl CPU {
    ld! {ldy_immediate, y, x, &AddressingMode::Immediate}
    ld! {ldy_zero_page, y, x, &AddressingMode::ZeroPage}
    ld! {ldy_zero_page_x, y, x, &AddressingMode::ZeroPageReg}
    ld! {ldy_absolute, y, x, &AddressingMode::Absolute}
    ld! {ldy_absolute_x, y, x, &AddressingMode::AbsoluteReg}
}

macro_rules! st {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident, $addr_mode: expr) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::ZeroPage, 3);
            cycles.insert(AddressingMode::ZeroPageReg, 4);
            cycles.insert(AddressingMode::Absolute, 4);
            cycles.insert(AddressingMode::AbsoluteReg, 5);
            cycles.insert(AddressingMode::IndirectX, 6);
            cycles.insert(AddressingMode::IndirectY, 6);

            match $addr_mode {
                AddressingMode::ZeroPage => memory.write_zero_page(&mut self.pc, self.$reg_name),
                AddressingMode::ZeroPageReg => {
                    memory.write_zero_page_x(&mut self.pc, self.$addr_reg, self.$reg_name)
                }
                AddressingMode::Absolute => memory.write_absolute(&mut self.pc, self.$reg_name),
                AddressingMode::AbsoluteReg => {
                    memory.write_absolute_x(&mut self.pc, self.$addr_reg, self.$reg_name)
                }
                AddressingMode::IndirectX => {
                    memory.write_indirect_x(&mut self.pc, self.x, self.$reg_name)
                }
                AddressingMode::IndirectY => {
                    memory.write_indirect_y(&mut self.pc, self.y, self.$reg_name)
                }
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            self.cycles += cycles[$addr_mode];
        }
    };
}

impl CPU {
    st! {sta_zero_page, a, x, &AddressingMode::ZeroPage}
    st! {sta_zero_page_x, a, x, &AddressingMode::ZeroPageReg}
    st! {sta_absolute, a, x, &AddressingMode::Absolute}
    st! {sta_absolute_x, a, x, &AddressingMode::AbsoluteReg}
    st! {sta_absolute_y, a, y, &AddressingMode::AbsoluteReg}
    st! {sta_indirect_x, a, x, &AddressingMode::IndirectX}
    st! {sta_indirect_y, a, y ,&AddressingMode::IndirectY}
}

impl CPU {
    st! {stx_zero_page, x, y, &AddressingMode::ZeroPage}
    st! {stx_zero_page_y, x, y, &AddressingMode::ZeroPageReg}
    st! {stx_absolute, x, y, &AddressingMode::Absolute}
}

impl CPU {
    st! {sty_zero_page, y, x, &AddressingMode::ZeroPage}
    st! {sty_zero_page_x, y, x, &AddressingMode::ZeroPageReg}
    st! {sty_absolute, y, x, &AddressingMode::Absolute}
}

macro_rules! transfer_reg_reg {
    ($func_name: ident, $reg_src: ident, $reg_dest: ident, $test_en: expr) => {
        fn $func_name(&mut self) {
            let value = self.$reg_src;
            self.$reg_dest = value;

            if $test_en {
                self.test_number(value);
            }
            self.cycles += 2;
        }
    };
}

impl CPU {
    transfer_reg_reg! {transfer_a_x, a, x, true}
    transfer_reg_reg! {transfer_a_y, a, y, true}
    transfer_reg_reg! {transfer_x_a, x, a, true}
    transfer_reg_reg! {transfer_y_a, y, a, true}
    transfer_reg_reg! {transfer_s_x, sp, x, true}
    transfer_reg_reg! {transfer_x_s, x, sp, false}
}

macro_rules! push_reg {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let value = self.$reg_name;
            memory.write_byte(0x0100 + self.sp as u16, value);

            self.sp -= 1;
            self.cycles += 3;
        }
    };
}

macro_rules! pull_reg {
    ($func_name: ident, $reg_name: ident, $test_en: expr) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            self.sp += 1;
            let value: u8 = memory.read(0x0100 + self.sp as u16);

            self.$reg_name = value;

            if $test_en {
                self.test_number(value);
            }

            self.cycles += 4;
        }
    };
}

impl CPU {
    push_reg! {push_accumulator, a}
    push_reg! {push_processor_status, status}
    pull_reg! {pull_accumulator, a, true}
    pull_reg! {pull_processor_status, status, false}
}

macro_rules! logic {
    ($func_name: ident, $op_func: expr, $reg_name: ident, $addr_mode: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Immediate, 2);
            cycles.insert(AddressingMode::ZeroPage, 3);
            cycles.insert(AddressingMode::ZeroPageReg, 4);
            cycles.insert(AddressingMode::Absolute, 4);
            cycles.insert(AddressingMode::AbsoluteReg, 4);
            cycles.insert(AddressingMode::IndirectX, 6);
            cycles.insert(AddressingMode::IndirectY, 5);

            let mut page_crossed = false;

            let value1 = self.a;
            let value2 = match $addr_mode {
                AddressingMode::Immediate => memory.read_immediate(&mut self.pc),
                AddressingMode::ZeroPage => memory.read_zero_page(&mut self.pc),
                AddressingMode::ZeroPageReg => memory.read_zero_page_x(&mut self.pc, self.x),
                AddressingMode::Absolute => memory.read_absolute(&mut self.pc),
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(
                    &mut self.pc,
                    self.$reg_name,
                    &mut page_crossed,
                ),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => {
                    memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed)
                }
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            self.a = $op_func(value1, value2);
            self.test_number(self.a);

            self.cycles += cycles[$addr_mode] + if page_crossed { 1 } else { 0 };
        }
    };
}

impl CPU {
    logic! {and_immediate, |n1, n2| n1 & n2, x, &AddressingMode::Immediate}
    logic! {and_zero_page, |n1, n2| n1 & n2, x, &AddressingMode::ZeroPage}
    logic! {and_zero_page_x, |n1, n2| n1 & n2, x, &AddressingMode::ZeroPageReg}
    logic! {and_absolute, |n1, n2| n1 & n2, x, &AddressingMode::Absolute}
    logic! {and_absolute_x, |n1, n2| n1 & n2, x, &AddressingMode::AbsoluteReg}
    logic! {and_absolute_y, |n1, n2| n1 & n2, y, &AddressingMode::AbsoluteReg}
    logic! {and_indirect_x, |n1, n2| n1 & n2, x, &AddressingMode::IndirectX}
    logic! {and_indirect_y, |n1, n2| n1 & n2, y, &AddressingMode::IndirectY}

    logic! {eor_immediate, |n1, n2| n1 ^ n2, x, &AddressingMode::Immediate}
    logic! {eor_zero_page, |n1, n2| n1 ^ n2, x, &AddressingMode::ZeroPage}
    logic! {eor_zero_page_x, |n1, n2| n1 ^ n2, x, &AddressingMode::ZeroPageReg}
    logic! {eor_absolute, |n1, n2| n1 ^ n2, x, &AddressingMode::Absolute}
    logic! {eor_absolute_x, |n1, n2| n1 ^ n2, x, &AddressingMode::AbsoluteReg}
    logic! {eor_absolute_y, |n1, n2| n1 ^ n2, y, &AddressingMode::AbsoluteReg}
    logic! {eor_indirect_x, |n1, n2| n1 ^ n2, x, &AddressingMode::IndirectX}
    logic! {eor_indirect_y, |n1, n2| n1 ^ n2, y, &AddressingMode::IndirectY}

    logic! {ora_immediate, |n1, n2| n1 | n2, x, &AddressingMode::Immediate}
    logic! {ora_zero_page, |n1, n2| n1 | n2, x, &AddressingMode::ZeroPage}
    logic! {ora_zero_page_x, |n1, n2| n1 | n2, x, &AddressingMode::ZeroPageReg}
    logic! {ora_absolute, |n1, n2| n1 | n2, x, &AddressingMode::Absolute}
    logic! {ora_absolute_x, |n1, n2| n1 | n2, x, &AddressingMode::AbsoluteReg}
    logic! {ora_absolute_y, |n1, n2| n1 | n2, y, &AddressingMode::AbsoluteReg}
    logic! {ora_indirect_x, |n1, n2| n1 | n2, x, &AddressingMode::IndirectX}
    logic! {ora_indirect_y, |n1, n2| n1 | n2, y, &AddressingMode::IndirectY}

    fn bit_zero_page(&mut self, memory: &Memory) {
        let value: u8 = memory.read_zero_page(&mut self.pc);

        self.set_zero((self.a & value) == 0);
        self.set_overflow((value & 0b0100_0000) == 0b0100_0000);
        self.set_negative((value & 0b1000_0000) == 0b1000_0000);

        self.cycles += 3;
    }

    fn bit_absolute(&mut self, memory: &Memory) {
        let value: u8 = memory.read_absolute(&mut self.pc);

        self.set_zero((self.a & value) == 0);
        self.set_overflow((value & 0b0100_0000) == 0b0100_0000);
        self.set_negative((value & 0b1000_0000) == 0b1000_0000);

        self.cycles += 4;
    }
}

macro_rules! arithmetic {
    ($func_name: ident, $arithm_func: expr, $addr_mode: expr, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Immediate, 2);
            cycles.insert(AddressingMode::ZeroPage, 3);
            cycles.insert(AddressingMode::ZeroPageReg, 4);
            cycles.insert(AddressingMode::Absolute, 4);
            cycles.insert(AddressingMode::AbsoluteReg, 4);
            cycles.insert(AddressingMode::IndirectX, 6);
            cycles.insert(AddressingMode::IndirectY, 5);

            let mut page_crossed = false;

            let value_reg = self.a;
            let value_mem = match $addr_mode {
                AddressingMode::Immediate => memory.read_immediate(&mut self.pc),
                AddressingMode::ZeroPage => memory.read_zero_page(&mut self.pc),
                AddressingMode::ZeroPageReg => memory.read_zero_page_x(&mut self.pc, self.x),
                AddressingMode::Absolute => memory.read_absolute(&mut self.pc),
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(
                    &mut self.pc,
                    self.$addr_reg,
                    &mut page_crossed,
                ),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => {
                    memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed)
                }
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };
            let carry = self.get_carry();

            self.cycles += cycles[$addr_mode] + if page_crossed { 1 } else { 0 };
            $arithm_func(self, value_reg, value_mem, carry);
        }
    };
}

macro_rules! cmp {
    ($func_name: ident, $reg_name: ident, $addr_mode: expr, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Immediate, 2);
            cycles.insert(AddressingMode::ZeroPage, 3);
            cycles.insert(AddressingMode::ZeroPageReg, 4);
            cycles.insert(AddressingMode::Absolute, 4);
            cycles.insert(AddressingMode::AbsoluteReg, 4);
            cycles.insert(AddressingMode::IndirectX, 6);
            cycles.insert(AddressingMode::IndirectY, 5);

            let mut page_crossed = false;

            let value_reg = self.$reg_name;
            let value_mem = match $addr_mode {
                AddressingMode::Immediate => memory.read_immediate(&mut self.pc),
                AddressingMode::ZeroPage => memory.read_zero_page(&mut self.pc),
                AddressingMode::ZeroPageReg => memory.read_zero_page_x(&mut self.pc, self.x),
                AddressingMode::Absolute => memory.read_absolute(&mut self.pc),
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(
                    &mut self.pc,
                    self.$addr_reg,
                    &mut page_crossed,
                ),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => {
                    memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed)
                }
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            let value = value_reg as i8 - value_mem as i8;

            self.set_zero(value == 0);
            self.set_negative(value < 0);
            self.set_carry(value >= 0);

            self.cycles += cycles[$addr_mode] + if page_crossed { 1 } else { 0 };
        }
    };
}

impl CPU {
    fn addition(cpu: &mut CPU, n1: u8, n2: u8, c: bool) {
        let value = n1 as i16 + n2 as i16 + (c as i16);
        cpu.a = value as u8;
        cpu.set_zero(value == 0);
        cpu.set_negative((value as i8) < 0);
        let carry = ((((n1 & 0b1000_0000) >> 7) == 1u8) || (((n2 & 0b1000_0000) >> 7) == 1u8))
            && ((value as u8 & 0b1000_0000) == 1u8);
        cpu.set_carry(carry);
        cpu.set_overflow(
            value < 0 && n1 > 0 && n2 > 0 || value > 0 && (n1 as i8) < 0 && (n2 as i8) < 0,
        );
    }

    fn substraction(cpu: &mut CPU, n1: u8, n2: u8, c: bool) {
        let value = n1 as i16 - n2 as i16 - (1 - (c as i16));
        cpu.a = value as u8;
        cpu.set_zero(value == 0);
        cpu.set_negative((value as i8) < 0);
        let carry = ((((n1 & 0b1000_0000) >> 7) == 1u8) || (((n2 & 0b1000_0000) >> 7) == 1u8))
            && ((value as u8 & 0b1000_0000) != 1u8);
        cpu.set_carry(carry);
        cpu.set_overflow(
            (value as i8) > 0 && (n1 as i8) < 0 && (n2 as i8) > 0
                || (value as i8) < 0 && (n1 as i8) > 0 && (n2 as i8) < 0,
        );
    }

    arithmetic! {adc_immediate, Self::addition, &AddressingMode::Immediate, x}
    arithmetic! {adc_zero_page, Self::addition, &AddressingMode::ZeroPage, x}
    arithmetic! {adc_zero_page_x, Self::addition, &AddressingMode::ZeroPageReg, x}
    arithmetic! {adc_absolute, Self::addition, &AddressingMode::Absolute, x}
    arithmetic! {adc_absolute_x, Self::addition, &AddressingMode::AbsoluteReg, x}
    arithmetic! {adc_absolute_y, Self::addition, &AddressingMode::AbsoluteReg, y}
    arithmetic! {adc_indirect_x, Self::addition, &AddressingMode::IndirectX, x}
    arithmetic! {adc_indirect_y, Self::addition, &AddressingMode::IndirectY, y}

    arithmetic! {sbc_immediate, Self::substraction, &AddressingMode::Immediate, x}
    arithmetic! {sbc_zero_page, Self::substraction, &AddressingMode::ZeroPage, x}
    arithmetic! {sbc_zero_page_x, Self::substraction, &AddressingMode::ZeroPageReg, x}
    arithmetic! {sbc_absolute, Self::substraction, &AddressingMode::Absolute, x}
    arithmetic! {sbc_absolute_x, Self::substraction, &AddressingMode::AbsoluteReg, x}
    arithmetic! {sbc_absolute_y, Self::substraction, &AddressingMode::AbsoluteReg, y}
    arithmetic! {sbc_indirect_x, Self::substraction, &AddressingMode::IndirectX, x}
    arithmetic! {sbc_indirect_y, Self::substraction, &AddressingMode::IndirectY, y}

    cmp! {cmp_immediate, a, &AddressingMode::Immediate, x}
    cmp! {cmp_zero_page, a, &AddressingMode::ZeroPage, x}
    cmp! {cmp_zero_page_x, a, &AddressingMode::ZeroPageReg, x}
    cmp! {cmp_absolute, a, &AddressingMode::Absolute, x}
    cmp! {cmp_absolute_x, a, &AddressingMode::AbsoluteReg, x}
    cmp! {cmp_absolute_y, a, &AddressingMode::AbsoluteReg, y}
    cmp! {cmp_indirect_x, a, &AddressingMode::IndirectX, x}
    cmp! {cmp_indirect_y, a, &AddressingMode::IndirectY, y}

    cmp! {cpx_immediate, x, &AddressingMode::Immediate, y}
    cmp! {cpx_zero_page, x, &AddressingMode::ZeroPage, y}
    cmp! {cpx_absolute, x, &AddressingMode::Absolute, y}

    cmp! {cpy_immediate, y, &AddressingMode::Immediate, x}
    cmp! {cpy_zero_page, y, &AddressingMode::ZeroPage, x}
    cmp! {cpy_absolute, y, &AddressingMode::Absolute, x}
}

macro_rules! inc_dec {
    ($func_name: ident, $op_func: expr, $addr_mode: expr, $reg_type: expr) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            match $reg_type {
                Register::X => {
                    self.x = $op_func(self.x) as u8;
                    self.cycles += 2;
                    self.test_number(self.x);
                }
                Register::Y => {
                    self.y = $op_func(self.y) as u8;
                    self.cycles += 2;
                    self.test_number(self.y);
                }
                _ => match $addr_mode {
                    AddressingMode::ZeroPage => {
                        let mut addr = self.pc;
                        let value: u8 = memory.read_zero_page(&mut addr);
                        let res = $op_func(value) as u8;
                        memory.write_zero_page(&mut self.pc, res);
                        self.cycles += 5;
                        self.test_number(res);
                    }
                    AddressingMode::ZeroPageReg => {
                        let mut addr = self.pc;
                        let value: u8 = memory.read_zero_page_x(&mut addr, self.x);
                        let res = $op_func(value) as u8;
                        memory.write_zero_page_x(&mut self.pc, self.x, res);
                        self.cycles += 6;
                        self.test_number(res);
                    }
                    AddressingMode::Absolute => {
                        let mut addr = self.pc;
                        let value: u8 = memory.read_absolute(&mut addr);
                        let res = $op_func(value) as u8;
                        memory.write_absolute(&mut self.pc, res);
                        self.cycles += 6;
                        self.test_number(res);
                    }
                    AddressingMode::AbsoluteReg => {
                        let mut addr = self.pc;
                        let value: u8 = memory.read_absolute_x(&mut addr, self.x);
                        let res = $op_func(value) as u8;
                        memory.write_absolute_x(&mut self.pc, self.x, res);
                        self.cycles += 7;
                        self.test_number(res);
                    }
                    _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                },
            }
        }
    };
}

impl CPU {
    inc_dec! {inc_zero_page, |n| n + 1, AddressingMode::ZeroPage, Register::None}
    inc_dec! {inc_zero_page_x, |n| n + 1, AddressingMode::ZeroPageReg, Register::None}
    inc_dec! {inc_absolute, |n| n + 1, AddressingMode::Absolute, Register::None}
    inc_dec! {inc_absolute_x, |n| n + 1, AddressingMode::AbsoluteReg, Register::None}
    inc_dec! {inx, |n| n + 1, AddressingMode::Implied, Register::X}
    inc_dec! {iny, |n| n + 1, AddressingMode::Implied, Register::Y}

    inc_dec! {dec_zero_page, |n| n as i8 - 1, AddressingMode::ZeroPage, Register::None}
    inc_dec! {dec_zero_page_x, |n| n as i8 - 1, AddressingMode::ZeroPageReg, Register::None}
    inc_dec! {dec_absolute, |n| n as i8 - 1, AddressingMode::Absolute, Register::None}
    inc_dec! {dec_absolute_x, |n| n as i8 - 1, AddressingMode::AbsoluteReg, Register::None}
    inc_dec! {dex, |n| n as i8 - 1, AddressingMode::Implied, Register::X}
    inc_dec! {dey, |n| n as i8 - 1, AddressingMode::Implied, Register::Y}
}

macro_rules! shifts {
    ($func_name: ident, $op_func: expr, $addr_mode: expr) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Implied, 2);
            cycles.insert(AddressingMode::ZeroPage, 5);
            cycles.insert(AddressingMode::ZeroPageReg, 6);
            cycles.insert(AddressingMode::Absolute, 6);
            cycles.insert(AddressingMode::AbsoluteReg, 7);

            // copy pc because read_xxx and write_xxx increment pc
            let mut pc = self.pc;
            let value = match $addr_mode {
                AddressingMode::Implied => self.a,
                AddressingMode::ZeroPage => memory.read_zero_page(&mut pc),
                AddressingMode::ZeroPageReg => memory.read_zero_page_x(&mut pc, self.x),
                AddressingMode::Absolute => memory.read_absolute(&mut pc),
                AddressingMode::AbsoluteReg => memory.read_absolute_x(&mut pc, self.x),
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            let (res, carry) = $op_func(value, self.get_carry());

            match $addr_mode {
                AddressingMode::Implied => self.a = res,
                AddressingMode::ZeroPage => memory.write_zero_page(&mut self.pc, res),
                AddressingMode::ZeroPageReg => memory.write_zero_page_x(&mut self.pc, self.x, res),
                AddressingMode::Absolute => memory.write_absolute(&mut self.pc, res),
                AddressingMode::AbsoluteReg => memory.write_absolute_x(&mut self.pc, self.x, res),
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            }

            self.set_carry(carry);
            self.test_number(res);
            self.cycles += cycles[$addr_mode];
        }
    };
}

fn asl_func(n: u8, _carry: bool) -> (u8, bool) {
    let bit7 = ((n & 0b1000_0000) >> 7) == 1;
    (n << 1, bit7)
}

fn lsr_func(n: u8, _carry: bool) -> (u8, bool) {
    let bit0 = (n & 0b0000_0001) == 1;
    ((n >> 1), bit0)
}

fn rol_func(n: u8, carry: bool) -> (u8, bool) {
    let bit7 = ((n & 0b1000_0000) >> 7) == 1;
    ((n << 1) & carry as u8, bit7)
}

fn ror_func(n: u8, carry: bool) -> (u8, bool) {
    let bit0 = (n & 0b0000_0001) == 1;
    ((n >> 1) & (carry as u8) << 7, bit0)
}

impl CPU {
    shifts! {asl_accumulator, asl_func, &AddressingMode::Implied}
    shifts! {asl_zero_page, asl_func, &AddressingMode::ZeroPage}
    shifts! {asl_zero_page_x, asl_func, &AddressingMode::ZeroPageReg}
    shifts! {asl_absolute, asl_func, &AddressingMode::Absolute}
    shifts! {asl_absolute_x, asl_func, &AddressingMode::AbsoluteReg}

    shifts! {lsr_accumulator, lsr_func, &AddressingMode::Implied}
    shifts! {lsr_zero_page, lsr_func, &AddressingMode::ZeroPage}
    shifts! {lsr_zero_page_x, lsr_func, &AddressingMode::ZeroPageReg}
    shifts! {lsr_absolute, lsr_func, &AddressingMode::Absolute}
    shifts! {lsr_absolute_x, lsr_func, &AddressingMode::AbsoluteReg}

    shifts! {rol_accumulator, rol_func, &AddressingMode::Implied}
    shifts! {rol_zero_page, rol_func, &AddressingMode::ZeroPage}
    shifts! {rol_zero_page_x, rol_func, &AddressingMode::ZeroPageReg}
    shifts! {rol_absolute, rol_func, &AddressingMode::Absolute}
    shifts! {rol_absolute_x, rol_func, &AddressingMode::AbsoluteReg}

    shifts! {ror_accumulator, ror_func, &AddressingMode::Implied}
    shifts! {ror_zero_page, ror_func, &AddressingMode::ZeroPage}
    shifts! {ror_zero_page_x, ror_func, &AddressingMode::ZeroPageReg}
    shifts! {ror_absolute, ror_func, &AddressingMode::Absolute}
    shifts! {ror_absolute_x, ror_func, &AddressingMode::AbsoluteReg}
}

macro_rules! jmp {
    ($func_name: ident, $addr_mode: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            use std::collections::HashMap;
            let mut cycles: HashMap<AddressingMode, u64> = HashMap::new();
            cycles.insert(AddressingMode::Absolute, 3);
            cycles.insert(AddressingMode::Indirect, 5);

            let instr_param = memory.read(self.pc);
            let pc_new = match $addr_mode {
                AddressingMode::Absolute => instr_param,
                AddressingMode::Indirect => memory.read(instr_param),
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
            };

            self.pc = pc_new;
            self.cycles += cycles[$addr_mode];
        }
    };
}

impl CPU {
    jmp! {jmp_absolute, &AddressingMode::Absolute}
    jmp! {jmp_indirect, &AddressingMode::Indirect}

    fn jsr_absolute(&mut self, memory: &mut Memory) {
        let pc = self.pc;
        let dest: u16 = memory.read(self.pc);
        memory.write(0x100u16 + self.sp as u16, pc + 2 - 1);
        self.sp -= 2;
        self.pc = dest;
        self.cycles += 6;
    }

    fn rts_implied(&mut self, memory: &Memory) {
        self.sp += 2;
        let dest: u16 = memory.read(0x100u16 + self.sp as u16);
        self.pc = dest;
        self.pc += 1;
        self.cycles += 6;
    }
}

macro_rules! branches {
    ($func_name: ident, $cpu_flag_val: ident, $is_set: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let offset: u8 = memory.read(self.pc);
            if self.$cpu_flag_val() == $is_set {
                self.cycles += (self.pc + offset as u16 > 0xff) as u64;
                self.pc += offset as u16;
                self.cycles += 3;
            } else {
                self.pc += 1;
                self.cycles += 2;
            }
        }
    };
}

impl CPU {
    branches! {bcc, get_carry, false}
    branches! {bcs, get_carry, true}
    branches! {beq, get_zero, true}
    branches! {bne, get_zero, false}
    branches! {bmi, get_negative, true}
    branches! {bpl, get_negative, false}
    branches! {bvc, get_overflow, false}
    branches! {bvs, get_overflow, true}
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    fn assert_cpu(cpu: &CPU, cpu_copy: &CPU) {
        assert_eq!(
            cpu.cycles, cpu_copy.cycles,
            "Amount of cycles used should be {}, but found {}",
            cpu_copy.cycles, cpu.cycles
        );
        assert_eq!(
            cpu.a, cpu_copy.a,
            "Register a value should be {}, but found {}",
            cpu_copy.a, cpu.a
        );
        assert_eq!(
            cpu.x, cpu_copy.x,
            "Register x value should be {}, but found {}",
            cpu_copy.x, cpu.x
        );
        assert_eq!(
            cpu.y, cpu_copy.y,
            "Register y value should be {}, but found {}",
            cpu_copy.y, cpu.y
        );
        assert_eq!(
            cpu.pc, cpu_copy.pc,
            "Program counter should be {}, but found {}",
            cpu_copy.pc, cpu.pc
        );
        assert_eq!(
            cpu.sp, cpu_copy.sp,
            "Stack pointer value should be {}, but found {}",
            cpu_copy.sp, cpu.sp
        );
        assert_eq!(
            cpu.get_carry(),
            cpu_copy.get_carry(),
            "Carry flag should be {}, but found {}!",
            cpu.get_carry(),
            cpu_copy.get_carry()
        );
        assert_eq!(
            cpu.get_zero(),
            cpu_copy.get_zero(),
            "Zero flag should be {}, but found {}!",
            cpu.get_zero(),
            cpu_copy.get_zero()
        );
        assert_eq!(
            cpu.get_interrupt_disable(),
            cpu_copy.get_interrupt_disable(),
            "Interrupt disable flag should be {}, but found {}!",
            cpu.get_interrupt_disable(),
            cpu_copy.get_interrupt_disable()
        );
        assert_eq!(
            cpu.get_decimal_mode(),
            cpu_copy.get_decimal_mode(),
            "Decimal mode flag should be {}, but found {}!",
            cpu.get_decimal_mode(),
            cpu_copy.get_decimal_mode()
        );
        assert_eq!(
            cpu.get_break_command(),
            cpu_copy.get_break_command(),
            "Break command flag should be {}, but found {}!",
            cpu.get_break_command(),
            cpu_copy.get_break_command()
        );
        assert_eq!(
            cpu.get_overflow(),
            cpu_copy.get_overflow(),
            "Overflow flag should be {}, but found {}!",
            cpu.get_overflow(),
            cpu_copy.get_overflow()
        );
        assert_eq!(
            cpu.get_negative(),
            cpu_copy.get_negative(),
            "Negative flag should be {}, but found {}!",
            cpu.get_negative(),
            cpu_copy.get_negative()
        );
    }

    macro_rules! test_ld {
        // reg_type is required for ABS_REG instructions only. For other instructions use Register::None
        ($func_name: ident, $reg_name: ident, $instr_name: expr, $addr_mode: expr, $reg_type: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Immediate, 2);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);
                pc_increments.insert(AddressingMode::IndirectX, 2);
                pc_increments.insert(AddressingMode::IndirectY, 2);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Immediate, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 3);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 4);
                cycles_increments.insert(AddressingMode::Absolute, 4);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 4);
                cycles_increments.insert(AddressingMode::IndirectX, 6);
                cycles_increments.insert(AddressingMode::IndirectY, 5);

                let mut additional_cycles = [0, 0, 0, 0];

                let values = [69u8, 0u8, 0xff, (!10u8 + 1)];
                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let y_values = [0x23, 0x00u8, 0x43, 0xBB];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_zp_final_y = [0x36u8, 0x69, 0x42, 0x66];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let addresses_absolute_final_y = [0x0036u16, 0xfff4, 0xAC10, 0x00BA];

                for i in 0..4 {
                    match $addr_mode {
                        AddressingMode::Immediate => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, values[i as usize]);
                        }
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values[i as usize]);
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory
                                .write(addresses_zp_final_x[i as usize] as u16, values[i as usize]);
                            memory
                                .write(addresses_zp_final_y[i as usize] as u16, values[i as usize]);
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values[i as usize]);
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory
                                .write(addresses_absolute_final_y[i as usize], values[i as usize]);
                            if $reg_type == Register::X {
                                memory.write(
                                    addresses_absolute_final_x[i as usize],
                                    values[i as usize],
                                );
                                if addresses_absolute_final_x[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            } else if $reg_type == Register::Y {
                                memory.write(
                                    addresses_absolute_final_y[i as usize],
                                    values[i as usize],
                                );
                                if addresses_absolute_final_y[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            }
                        }
                        AddressingMode::IndirectX => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory.write(addresses_absolute[i as usize], values[i as usize]);
                        }
                        AddressingMode::IndirectY => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory
                                .write(addresses_absolute_final_y[i as usize], values[i as usize]);
                            if addresses_absolute_final_y[i as usize] > 0xff {
                                additional_cycles[i as usize] += 1;
                            }
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..4 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.x = x_values[i];
                    cpu.y = y_values[i];
                    cpu.execute(&mut memory, instruction);

                    cpu_copy.x = x_values[i];
                    cpu_copy.y = y_values[i];
                    cpu_copy.$reg_name = value;
                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode] + additional_cycles[i];
                    cpu_copy.set_zero(value == 0);
                    cpu_copy.set_negative((value as i8) < 0);
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    // LDA
    test_ld! {test_lda_immediate, a, Instruction::LDA_IM, &AddressingMode::Immediate, Register::None}
    test_ld! {test_lda_zero_page, a, Instruction::LDA_ZP, &AddressingMode::ZeroPage, Register::None}
    test_ld! {test_lda_zero_page_x, a, Instruction::LDA_ZP_X, &AddressingMode::ZeroPageReg, Register::None}
    test_ld! {test_lda_absolute, a, Instruction::LDA_ABS, &AddressingMode::Absolute, Register::None}
    test_ld! {test_lda_absolute_x, a, Instruction::LDA_ABS_X, &AddressingMode::AbsoluteReg, Register::X}
    test_ld! {test_lda_absolute_y, a, Instruction::LDA_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y}
    test_ld! {test_lda_indirect_x, a, Instruction::LDA_IN_X, &AddressingMode::IndirectX, Register::None}
    test_ld! {test_lda_indirect_y, a, Instruction::LDA_IN_Y, &AddressingMode::IndirectY, Register::None}

    // LDX
    test_ld! {test_ldx_immediate, x, Instruction::LDX_IM, &AddressingMode::Immediate, Register::None}
    test_ld! {test_ldx_zero_page, x, Instruction::LDX_ZP, &AddressingMode::ZeroPage, Register::None}
    test_ld! {test_ldx_zero_page_y, x, Instruction::LDX_ZP_Y, &AddressingMode::ZeroPageReg, Register::None}
    test_ld! {test_ldx_absolute, x, Instruction::LDX_ABS, &AddressingMode::Absolute, Register::None}
    test_ld! {test_ldx_absolute_y, x, Instruction::LDX_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y}

    // LDY
    test_ld! {test_ldy_immediate, y, Instruction::LDY_IM, &AddressingMode::Immediate, Register::None}
    test_ld! {test_ldy_zero_page, y, Instruction::LDY_ZP, &AddressingMode::ZeroPage, Register::None}
    test_ld! {test_ldy_zero_page_x, y, Instruction::LDY_ZP_X, &AddressingMode::ZeroPageReg, Register::None}
    test_ld! {test_ldy_absolute, y, Instruction::LDY_ABS, &AddressingMode::Absolute, Register::None}
    test_ld! {test_ldy_absolute_x, y, Instruction::LDY_ABS_X, &AddressingMode::AbsoluteReg, Register::X}

    macro_rules! test_st {
        ($func_name: ident, $reg_name: ident, $instr_name: expr, $addr_mode: expr, $reg_type: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);
                pc_increments.insert(AddressingMode::IndirectX, 2);
                pc_increments.insert(AddressingMode::IndirectY, 2);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::ZeroPage, 3);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 4);
                cycles_increments.insert(AddressingMode::Absolute, 4);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 5);
                cycles_increments.insert(AddressingMode::IndirectX, 6);
                cycles_increments.insert(AddressingMode::IndirectY, 6);

                let values = [69u8, 0u8, 0xff, (!10u8 + 1)];
                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let y_values = [0x23, 0x00u8, 0x43, 0xBB];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_zp_final_y = [0x36u8, 0x69, 0x42, 0x66];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let addresses_absolute_final_y = [0x0036u16, 0xfff4, 0xAC10, 0x00BA];
                let mut addresses_final: [u16; 4] = [0, 0, 0, 0];

                for i in 0..4 {
                    match $addr_mode {
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            addresses_final[i as usize] = addresses_zp[i as usize] as u16;
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            if $reg_type == Register::X {
                                addresses_final[i as usize] =
                                    addresses_zp_final_x[i as usize] as u16;
                            } else if $reg_type == Register::Y {
                                addresses_final[i as usize] =
                                    addresses_zp_final_y[i as usize] as u16;
                            }
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            addresses_final[i as usize] = addresses_absolute[i as usize];
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            if $reg_type == Register::X {
                                addresses_final[i as usize] =
                                    addresses_absolute_final_x[i as usize];
                            } else if $reg_type == Register::Y {
                                addresses_final[i as usize] =
                                    addresses_absolute_final_y[i as usize];
                            }
                        }
                        AddressingMode::IndirectX => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            addresses_final[i as usize] = addresses_absolute[i as usize];
                        }
                        AddressingMode::IndirectY => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            addresses_final[i as usize] = addresses_absolute_final_y[i as usize];
                        }
                        _ => {
                            panic!("Unsupported addressing mode {:?}!", $addr_mode);
                        }
                    }
                }

                for i in 0..4 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let address = addresses_final[i];
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.x = x_values[i];
                    cpu.y = y_values[i];
                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(address as u16);

                    assert_eq!(actual_value, value);
                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode];
                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.x = x_values[i];
                    cpu_copy.y = y_values[i];
                    cpu_copy.$reg_name = value;
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    // STA
    test_st! {test_sta_zero_page, a, Instruction::STA_ZP, &AddressingMode::ZeroPage, Register::None}
    test_st! {test_sta_zero_page_x, a, Instruction::STA_ZP_X, &AddressingMode::ZeroPageReg, Register::X}
    test_st! {test_sta_absolute, a, Instruction::STA_ABS, &AddressingMode::Absolute, Register::None}
    test_st! {test_sta_absolute_x, a, Instruction::STA_ABS_X, &AddressingMode::AbsoluteReg, Register::X}
    test_st! {test_sta_absolute_y, a, Instruction::STA_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y}
    test_st! {test_sta_indirect_x, a, Instruction::STA_IN_X, &AddressingMode::IndirectX, Register::None}
    test_st! {test_sta_indirect_y, a, Instruction::STA_IN_Y, &AddressingMode::IndirectY, Register::None}
    // STX
    test_st! {test_stx_zero_page, x, Instruction::STX_ZP, &AddressingMode::ZeroPage, Register::None}
    test_st! {test_stx_zero_page_y, x, Instruction::STX_ZP_Y, &AddressingMode::ZeroPageReg, Register::Y}
    test_st! {test_stx_absolute, x, Instruction::STX_ABS, &AddressingMode::Absolute, Register::None}
    // STY
    test_st! {test_sty_zero_page, y, Instruction::STY_ZP, &AddressingMode::ZeroPage, Register::None}
    test_st! {test_sty_zero_page_x, y, Instruction::STY_ZP_X, &AddressingMode::ZeroPageReg, Register::X}
    test_st! {test_sty_absolute, y, Instruction::STY_ABS, &AddressingMode::Absolute, Register::None}

    // Transfer
    macro_rules! test_transfer_reg_reg {
        ($func_name: ident, $reg_src: ident, $reg_dest: ident, $instr_name: expr, $test_flags_en: expr) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let values = [0u8, 69, (!105u8 + 1)];

                for i in 0..3 {
                    memory.write_byte(i, $instr_name.into());
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_src = value;

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.cycles = cycles + 2;
                    cpu_copy.pc = pc + 1;
                    cpu_copy.$reg_src = value;
                    cpu_copy.$reg_dest = value;
                    if $test_flags_en {
                        cpu_copy.set_zero(value == 0);
                        cpu_copy.set_negative((value as i8) < 0);
                    }
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    test_transfer_reg_reg! {test_transfer_a_x, a, x, Instruction::TAX, true}
    test_transfer_reg_reg! {test_transfer_a_y, a, y, Instruction::TAY, true}
    test_transfer_reg_reg! {test_transfer_x_a, x, a, Instruction::TXA, true}
    test_transfer_reg_reg! {test_transfer_y_a, y, a, Instruction::TYA, true}
    test_transfer_reg_reg! {test_transfer_s_x, sp, x, Instruction::TSX, true}
    test_transfer_reg_reg! {test_transfer_x_s, x, sp, Instruction::TXS, false}

    // Stack
    macro_rules! test_push_stack {
        ($func_name: ident, $reg_name: ident, $instr_name: expr) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                cpu.sp = 0xff;

                let mut cpu_copy = cpu.clone();

                let values = [0u8, 69, (!105u8 + 1)];

                for i in 0..3 {
                    memory.write_byte(i, $instr_name.into());
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(0x0100 + (cpu.sp + 1) as u16);

                    cpu_copy.cycles = cycles + 3;
                    cpu_copy.pc = pc + 1;
                    cpu_copy.$reg_name = value;
                    cpu_copy.sp = 0xff - (i + 1) as u8;
                    assert_cpu(&cpu, &cpu_copy);
                    assert_eq!(actual_value, value);
                }
            }
        };
    }

    macro_rules! test_pull_stack {
        ($func_name: ident, $reg_name: ident, $instr_name: expr, $instr_push_name: expr) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                cpu.sp = 0xff;

                let values = [0u8, 69, (!105u8 + 1)];

                for i in 0..3 {
                    memory.write_byte(i, $instr_name.into());
                    cpu.$reg_name = values[i as usize];
                    cpu.execute(&mut memory, $instr_push_name.into());
                }
                let mut cpu_copy = cpu.clone();
                let sp_init = cpu_copy.sp;

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[values.len() - i - 1];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.cycles = cycles + 4;
                    cpu_copy.pc = pc + 1;
                    cpu_copy.sp = sp_init + i as u8 + 1;
                    cpu_copy.$reg_name = value;
                    if (u8::from(Instruction::PLA) == u8::from($instr_name)) {
                        cpu_copy.set_zero(value == 0);
                        cpu_copy.set_negative((value as i8) < 0);
                    }
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    test_push_stack! {test_push_accumulator, a, Instruction::PHA}
    test_push_stack! {test_push_processor_status, status, Instruction::PHP}
    test_pull_stack! {test_pull_accumulator, a, Instruction::PLA, Instruction::PHA}
    test_pull_stack! {test_pull_processor_status, status, Instruction::PLP, Instruction::PHP}

    macro_rules! test_logic {
        ($func_name: ident, $instr_name: expr, $addr_mode: expr, $reg_type: expr, $op_func: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Immediate, 2);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);
                pc_increments.insert(AddressingMode::IndirectX, 2);
                pc_increments.insert(AddressingMode::IndirectY, 2);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Immediate, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 3);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 4);
                cycles_increments.insert(AddressingMode::Absolute, 4);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 4);
                cycles_increments.insert(AddressingMode::IndirectX, 6);
                cycles_increments.insert(AddressingMode::IndirectY, 5);

                let mut additional_cycles = [0, 0, 0, 0];

                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let y_values = [0x23, 0x00u8, 0x43, 0xBB];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let addresses_absolute_final_y = [0x0036u16, 0xfff4, 0xAC10, 0x00BA];

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111, 0b1010_1010];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011, 0b0101_1011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();

                for i in 0..4 {
                    match $addr_mode {
                        AddressingMode::Immediate => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, values2[i as usize]);
                        }
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values2[i as usize]);
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                values2[i as usize],
                            );
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values2[i as usize]);
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            if $reg_type == Register::X {
                                memory.write(
                                    addresses_absolute_final_x[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_x[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            } else if $reg_type == Register::Y {
                                memory.write(
                                    addresses_absolute_final_y[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_y[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            }
                        }
                        AddressingMode::IndirectX => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory.write(addresses_absolute[i as usize], values2[i as usize])
                        }
                        AddressingMode::IndirectY => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory
                                .write(addresses_absolute_final_y[i as usize], values2[i as usize]);
                            if addresses_absolute_final_y[i as usize] > 0xff {
                                additional_cycles[i as usize] += 1;
                            }
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..4 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];
                    cpu.x = x_values[i];
                    cpu.y = y_values[i];

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode] + additional_cycles[i];
                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.a = value;
                    cpu_copy.x = x_values[i];
                    cpu_copy.y = y_values[i];
                    cpu_copy.set_zero(value == 0);
                    cpu_copy.set_negative((value as i8) < 0);
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    // AND
    test_logic! {test_and_immediate, Instruction::AND_IM, &AddressingMode::Immediate, Register::None, |n1, n2| n1 & n2}
    test_logic! {test_and_zero_page, Instruction::AND_ZP, &AddressingMode::ZeroPage, Register::None, |n1, n2| n1 & n2}
    test_logic! {test_and_zero_page_x, Instruction::AND_ZP_X, &AddressingMode::ZeroPageReg, Register::X, |n1, n2| n1 & n2}
    test_logic! {test_and_absolute, Instruction::AND_ABS, &AddressingMode::Absolute, Register::None, |n1, n2| n1 & n2}
    test_logic! {test_and_absolute_x, Instruction::AND_ABS_X, &AddressingMode::AbsoluteReg, Register::X, |n1, n2| n1 & n2}
    test_logic! {test_and_absolute_y, Instruction::AND_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y, |n1, n2| n1 & n2}
    test_logic! {test_and_indirect_x, Instruction::AND_IN_X, &AddressingMode::IndirectX, Register::X, |n1, n2| n1 & n2}
    test_logic! {test_and_indirect_y, Instruction::AND_IN_Y, &AddressingMode::IndirectY, Register::Y, |n1, n2| n1 & n2}
    // EOR
    test_logic! {test_eor_immediate, Instruction::EOR_IM, &AddressingMode::Immediate, Register::None, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_zero_page, Instruction::EOR_ZP, &AddressingMode::ZeroPage, Register::None, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_zero_page_x, Instruction::EOR_ZP_X, &AddressingMode::ZeroPageReg, Register::X, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_absolute, Instruction::EOR_ABS, &AddressingMode::Absolute, Register::None, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_absolute_x, Instruction::EOR_ABS_X, &AddressingMode::AbsoluteReg, Register::X, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_absolute_y, Instruction::EOR_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_indirect_x, Instruction::EOR_IN_X, &AddressingMode::IndirectX, Register::X, |n1, n2| n1 ^ n2}
    test_logic! {test_eor_indirect_y, Instruction::EOR_IN_Y, &AddressingMode::IndirectY, Register::Y, |n1, n2| n1 ^ n2}
    // ORA
    test_logic! {test_ora_immediate, Instruction::ORA_IM, &AddressingMode::Immediate, Register::None, |n1, n2| n1 | n2}
    test_logic! {test_ora_zero_page, Instruction::ORA_ZP, &AddressingMode::ZeroPage, Register::None, |n1, n2| n1 | n2}
    test_logic! {test_ora_zero_page_x, Instruction::ORA_ZP_X, &AddressingMode::ZeroPageReg, Register::X, |n1, n2| n1 | n2}
    test_logic! {test_ora_absolute, Instruction::ORA_ABS, &AddressingMode::Absolute, Register::None, |n1, n2| n1 | n2}
    test_logic! {test_ora_absolute_x, Instruction::ORA_ABS_X, &AddressingMode::AbsoluteReg, Register::X, |n1, n2| n1 | n2}
    test_logic! {test_ora_absolute_y, Instruction::ORA_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y, |n1, n2| n1 | n2}
    test_logic! {test_ora_indirect_x, Instruction::ORA_IN_X, &AddressingMode::IndirectX, Register::X, |n1, n2| n1 | n2}
    test_logic! {test_ora_indirect_y, Instruction::ORA_IN_Y, &AddressingMode::IndirectY, Register::Y, |n1, n2| n1 | n2}

    macro_rules! test_compare {
        ($func_name: ident, $reg_name: ident, $instr_name: expr, $addr_mode: expr, $reg_type: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Immediate, 2);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);
                pc_increments.insert(AddressingMode::IndirectX, 2);
                pc_increments.insert(AddressingMode::IndirectY, 2);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Immediate, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 3);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 4);
                cycles_increments.insert(AddressingMode::Absolute, 4);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 4);
                cycles_increments.insert(AddressingMode::IndirectX, 6);
                cycles_increments.insert(AddressingMode::IndirectY, 5);

                let mut additional_cycles = [0, 0, 0, 0];

                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let y_values = [0x23, 0x00u8, 0x43, 0xBB];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let addresses_absolute_final_y = [0x0036u16, 0xfff4, 0xAC10, 0x00BA];

                let values1 = [123u8, 53, 234, 10];
                let values2 = [123u8, 69, 13, 255];
                let values_res = [0u8, 246, 221, 11];

                for i in 0..3 {
                    match $addr_mode {
                        AddressingMode::Immediate => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, values2[i as usize]);
                        }
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values2[i as usize]);
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                values2[i as usize],
                            );
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values2[i as usize]);
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            if $reg_type == Register::X {
                                memory.write(
                                    addresses_absolute_final_x[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_x[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            } else if $reg_type == Register::Y {
                                memory.write(
                                    addresses_absolute_final_y[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_y[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            }
                        }
                        AddressingMode::IndirectX => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory.write(addresses_absolute[i as usize], values2[i as usize])
                        }
                        AddressingMode::IndirectY => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory
                                .write(addresses_absolute_final_y[i as usize], values2[i as usize]);
                            if addresses_absolute_final_y[i as usize] > 0xff {
                                additional_cycles[i as usize] += 1;
                            }
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.x = x_values[i];
                    cpu.y = y_values[i];
                    cpu.$reg_name = values1[i];

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode] + additional_cycles[i];
                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.x = x_values[i];
                    cpu_copy.y = y_values[i];
                    cpu_copy.$reg_name = values1[i];
                    cpu_copy.set_carry((value as i8) >= 0);
                    cpu_copy.set_zero(value == 0);
                    cpu_copy.set_negative((value as i8) < 0);
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }
    // CMP
    test_compare! {test_cmp_immediate, a, Instruction::CMP_IM, &AddressingMode::Immediate, Register::None}
    test_compare! {test_cmp_zero_page, a, Instruction::CMP_ZP, &AddressingMode::ZeroPage, Register::None}
    test_compare! {test_cmp_zero_page_x, a, Instruction::CMP_ZP_X, &AddressingMode::ZeroPageReg, Register::X}
    test_compare! {test_cmp_absolute, a, Instruction::CMP_ABS, &AddressingMode::Absolute, Register::None}
    test_compare! {test_cmp_absolute_x, a, Instruction::CMP_ABS_X, &AddressingMode::AbsoluteReg, Register::X}
    test_compare! {test_cmp_absolute_y, a, Instruction::CMP_ABS_Y, &AddressingMode::AbsoluteReg, Register::Y}
    test_compare! {test_cmp_indirect_x, a, Instruction::CMP_IN_X, &AddressingMode::IndirectX, Register::X}
    test_compare! {test_cmp_indirect_y, a, Instruction::CMP_IN_Y, &AddressingMode::IndirectY, Register::Y}
    // CPX
    test_compare! {test_cpx_immediate, x, Instruction::CPX_IM, &AddressingMode::Immediate, Register::None}
    test_compare! {test_cpx_zero_page, x, Instruction::CPX_ZP, &AddressingMode::ZeroPage, Register::None}
    test_compare! {test_cpx_absolute, x, Instruction::CPX_ABS, &AddressingMode::Absolute, Register::None}
    // CPY
    test_compare! {test_cpy_immediate, y, Instruction::CPY_IM, &AddressingMode::Immediate, Register::None}
    test_compare! {test_cpy_zero_page, y, Instruction::CPY_ZP, &AddressingMode::ZeroPage, Register::None}
    test_compare! {test_cpy_absolute, y, Instruction::CPY_ABS, &AddressingMode::Absolute, Register::None}

    enum ArithmeticOperation {
        Addition,
        Substraction,
    }

    macro_rules! test_arithmetic {
        ($func_name: ident, $instr_name: expr, $op_type: expr, $addr_mode: expr, $reg_type: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Immediate, 2);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);
                pc_increments.insert(AddressingMode::IndirectX, 2);
                pc_increments.insert(AddressingMode::IndirectY, 2);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Immediate, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 3);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 4);
                cycles_increments.insert(AddressingMode::Absolute, 4);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 4);
                cycles_increments.insert(AddressingMode::IndirectX, 6);
                cycles_increments.insert(AddressingMode::IndirectY, 5);

                let mut additional_cycles = [0, 0, 0, 0];

                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let y_values = [0x23, 0x00u8, 0x43, 0xBB];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let addresses_absolute_final_y = [0x0036u16, 0xfff4, 0xAC10, 0x00BA];

                let overflow_flags = [true, false, true, false, true];
                let values1 = [123u8, 152, 234, 10, 128];
                let values2 = [123u8, 167, 13, 255, 127];
                let addition: [(u8, bool); 5] = [
                    (247u8, false),
                    (63, true),
                    (248, false),
                    (9, true),
                    (0, true),
                ];
                let substraction: [(u8, bool); 5] = [
                    (0u8, false),
                    (240, false),
                    (221, false),
                    (10, false),
                    (129, true),
                ];

                let operation = match $op_type {
                    ArithmeticOperation::Addition => addition,
                    ArithmeticOperation::Substraction => substraction,
                };

                for i in 0..3 {
                    match $addr_mode {
                        AddressingMode::Immediate => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, values2[i as usize]);
                        }
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values2[i as usize]);
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                values2[i as usize],
                            );
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values2[i as usize]);
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            if $reg_type == Register::X {
                                memory.write(
                                    addresses_absolute_final_x[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_x[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            } else if $reg_type == Register::Y {
                                memory.write(
                                    addresses_absolute_final_y[i as usize],
                                    values2[i as usize],
                                );
                                if addresses_absolute_final_y[i as usize] > 0xff {
                                    additional_cycles[i as usize] += 1;
                                }
                            }
                        }
                        AddressingMode::IndirectX => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp_final_x[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory.write(addresses_absolute[i as usize], values2[i as usize])
                        }
                        AddressingMode::IndirectY => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(
                                addresses_zp[i as usize] as u16,
                                addresses_absolute[i as usize],
                            );
                            memory
                                .write(addresses_absolute_final_y[i as usize], values2[i as usize]);
                            if addresses_absolute_final_y[i as usize] > 0xff {
                                additional_cycles[i as usize] += 1;
                            }
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = operation[i].0;
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.x = x_values[i];
                    cpu.y = y_values[i];
                    cpu.a = values1[i];

                    cpu.set_carry(overflow_flags[i]);

                    cpu.execute(&mut memory, instruction);

                    let carry = match $op_type {
                        ArithmeticOperation::Addition => {
                            ((((values1[i] & 0b1000_0000) >> 7) == 1u8)
                                || (((values2[i] & 0b1000_0000) >> 7) == 1u8))
                                && ((value as u8 & 0b1000_0000) == 1u8)
                        }
                        ArithmeticOperation::Substraction => {
                            ((((values1[i] & 0b1000_0000) >> 7) == 1u8)
                                || (((values2[i] & 0b1000_0000) >> 7) == 1u8))
                                && ((value as u8 & 0b1000_0000) != 1u8)
                        }
                    };

                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode] + additional_cycles[i];
                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.x = x_values[i];
                    cpu_copy.y = y_values[i];
                    cpu_copy.a = value;
                    cpu_copy.set_carry(carry);
                    cpu_copy.set_zero(value == 0);
                    cpu_copy.set_overflow(operation[i].1);
                    cpu_copy.set_negative((value as i8) < 0);
                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }
    // ADC
    test_arithmetic! {test_adc_immediate, Instruction::ADC_IM, ArithmeticOperation::Addition, &AddressingMode::Immediate, Register::None}
    test_arithmetic! {test_adc_zero_page, Instruction::ADC_ZP, ArithmeticOperation::Addition, &AddressingMode::ZeroPage, Register::None}
    test_arithmetic! {test_adc_zero_page_x, Instruction::ADC_ZP_X, ArithmeticOperation::Addition, &AddressingMode::ZeroPageReg, Register::X}
    test_arithmetic! {test_adc_absolute, Instruction::ADC_ABS, ArithmeticOperation::Addition, &AddressingMode::Absolute, Register::None}
    test_arithmetic! {test_adc_absolute_x, Instruction::ADC_ABS_X, ArithmeticOperation::Addition, &AddressingMode::AbsoluteReg, Register::X}
    test_arithmetic! {test_adc_absolute_y, Instruction::ADC_ABS_Y, ArithmeticOperation::Addition, &AddressingMode::AbsoluteReg, Register::Y}
    test_arithmetic! {test_adc_indirect_x, Instruction::ADC_IN_X, ArithmeticOperation::Addition, &AddressingMode::IndirectX, Register::X}
    test_arithmetic! {test_adc_indirect_y, Instruction::ADC_IN_Y, ArithmeticOperation::Addition, &AddressingMode::IndirectY, Register::Y}

    // SBC
    test_arithmetic! {test_sbc_immediate, Instruction::SBC_IM, ArithmeticOperation::Substraction, &AddressingMode::Immediate, Register::None}
    test_arithmetic! {test_sbc_zero_page, Instruction::SBC_ZP, ArithmeticOperation::Substraction, &AddressingMode::ZeroPage, Register::None}
    test_arithmetic! {test_sbc_zero_page_x, Instruction::SBC_ZP_X, ArithmeticOperation::Substraction, &AddressingMode::ZeroPageReg, Register::X}
    test_arithmetic! {test_sbc_absolute, Instruction::SBC_ABS, ArithmeticOperation::Substraction, &AddressingMode::Absolute, Register::None}
    test_arithmetic! {test_sbc_absolute_x, Instruction::SBC_ABS_X, ArithmeticOperation::Substraction, &AddressingMode::AbsoluteReg, Register::X}
    test_arithmetic! {test_sbc_absolute_y, Instruction::SBC_ABS_Y, ArithmeticOperation::Substraction, &AddressingMode::AbsoluteReg, Register::Y}
    test_arithmetic! {test_sbc_indirect_x, Instruction::SBC_IN_X, ArithmeticOperation::Substraction, &AddressingMode::IndirectX, Register::X}
    test_arithmetic! {test_sbc_indirect_y, Instruction::SBC_IN_Y, ArithmeticOperation::Substraction, &AddressingMode::IndirectY, Register::Y}

    macro_rules! test_increments_decrements {
        ($func_name: ident, $instr_name: expr, $op_func: expr, $reg_type: expr, $addr_mode: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Implied, 1);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Implied, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 5);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 6);
                cycles_increments.insert(AddressingMode::Absolute, 6);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 7);

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let values = [69, 0, 234, 1];
                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let mut addresses_final = [0, 0, 0, 0];

                let values_res = values.map(|value| $op_func(value));

                for i in 0..4 {
                    match $addr_mode {
                        AddressingMode::Implied => memory.write(i, u8::from($instr_name)),
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values[i as usize]);
                            addresses_final[i as usize] = addresses_zp[i as usize] as u16;
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory
                                .write(addresses_zp_final_x[i as usize] as u16, values[i as usize]);
                            addresses_final[i as usize] = addresses_zp_final_x[i as usize] as u16;
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values[i as usize]);
                            addresses_final[i as usize] = addresses_absolute[i as usize];
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory
                                .write(addresses_absolute_final_x[i as usize], values[i as usize]);
                            addresses_final[i as usize] = addresses_absolute_final_x[i as usize];
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..4 {
                    let value: u8 = values_res[i] as u8;
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);

                    if $reg_type == Register::X {
                        cpu.x = values[i];
                    } else if $reg_type == Register::Y {
                        cpu.y = values[i];
                    } else {
                        cpu.x = x_values[i];
                    }

                    cpu.execute(&mut memory, instruction);

                    let res: u8 = memory.read(addresses_final[i as usize]);

                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode];

                    if $reg_type == Register::None {
                        cpu_copy.x = x_values[i];
                    }

                    if $reg_type == Register::X {
                        cpu_copy.x = value;
                    } else if $reg_type == Register::Y {
                        cpu_copy.y = value;
                    }
                    cpu_copy.set_zero(value == 0);
                    cpu_copy.set_negative((value as i8) < 0);
                    assert_cpu(&cpu, &cpu_copy);
                    if $reg_type != Register::X && $reg_type != Register::Y {
                        assert_eq!(res, values_res[i] as u8);
                    }
                }
            }
        };
    }

    test_increments_decrements! {test_inc_zero_page, Instruction::INC_ZP, |n| n + 1, Register::None, &AddressingMode::ZeroPage}
    test_increments_decrements! {test_inc_zero_page_x, Instruction::INC_ZP_X, |n| n + 1, Register::None, &AddressingMode::ZeroPageReg}
    test_increments_decrements! {test_inc_absolute, Instruction::INC_ABS, |n| n + 1, Register::None, &AddressingMode::Absolute}
    test_increments_decrements! {test_inc_absolute_x, Instruction::INC_ABS_X, |n| n + 1, Register::None, &AddressingMode::AbsoluteReg}
    test_increments_decrements! {test_inx, Instruction::INX, |n| n + 1, Register::X, &AddressingMode::Implied}
    test_increments_decrements! {test_iny, Instruction::INY, |n| n + 1, Register::Y, &AddressingMode::Implied}

    test_increments_decrements! {test_dec_zero_page, Instruction::DEC_ZP, |n| n as i8 - 1, Register::None, &AddressingMode::ZeroPage}
    test_increments_decrements! {test_dec_zero_page_x, Instruction::DEC_ZP_X, |n| n as i8 - 1, Register::None, &AddressingMode::ZeroPageReg}
    test_increments_decrements! {test_dec_absolute, Instruction::DEC_ABS, |n| n as i8 - 1, Register::None, &AddressingMode::Absolute}
    test_increments_decrements! {test_dec_absolute_x, Instruction::DEC_ABS_X, |n| n as i8 - 1, Register::None, &AddressingMode::AbsoluteReg}
    test_increments_decrements! {test_dex, Instruction::DEX, |n| n as i8 - 1, Register::X, &AddressingMode::Implied}
    test_increments_decrements! {test_dey, Instruction::DEY, |n| n as i8 - 1, Register::Y, &AddressingMode::Implied}

    macro_rules! test_shifts {
        ($func_name: ident, $instr_name: expr, $op_func: expr, $addr_mode: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                let mut pc_increments: HashMap<AddressingMode, u16> = HashMap::new();
                pc_increments.insert(AddressingMode::Implied, 1);
                pc_increments.insert(AddressingMode::ZeroPage, 2);
                pc_increments.insert(AddressingMode::ZeroPageReg, 2);
                pc_increments.insert(AddressingMode::Absolute, 3);
                pc_increments.insert(AddressingMode::AbsoluteReg, 3);

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Implied, 2);
                cycles_increments.insert(AddressingMode::ZeroPage, 5);
                cycles_increments.insert(AddressingMode::ZeroPageReg, 6);
                cycles_increments.insert(AddressingMode::Absolute, 6);
                cycles_increments.insert(AddressingMode::AbsoluteReg, 7);

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let carries = [true, false, true, false];
                let values = [69, 0, 234, 1];
                let addresses_zp = [0x13u8, 0x69, 0xFF, 0xAB];
                let x_values = [0x10, 0x00u8, 0x16, 0x4A];
                let addresses_zp_final_x = [0x23u8, 0x69, 0x15, 0xF5];
                let addresses_absolute = [0x0013u16, 0xfff4, 0xABCD, 0xffff];
                let addresses_absolute_final_x = [0x0023u16, 0xfff4, 0xABE3, 0x0049];
                let mut addresses_final = [0, 0, 0, 0];

                let values_res: Vec<(u8, bool)> = zip(values, carries).map(|value_and_carry| $op_func(value_and_carry.0, value_and_carry.1)).collect();

                for i in 0..3 {
                    match $addr_mode {
                        AddressingMode::Implied => memory.write(i, u8::from($instr_name)),
                        AddressingMode::ZeroPage => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory.write(addresses_zp[i as usize] as u16, values[i as usize]);
                            addresses_final[i as usize] = addresses_zp[i as usize] as u16;
                        }
                        AddressingMode::ZeroPageReg => {
                            memory.write(2 * i, u8::from($instr_name));
                            memory.write(2 * i + 1, addresses_zp[i as usize]);
                            memory
                                .write(addresses_zp_final_x[i as usize] as u16, values[i as usize]);
                            addresses_final[i as usize] = addresses_zp_final_x[i as usize] as u16;
                        }
                        AddressingMode::Absolute => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory.write(addresses_absolute[i as usize], values[i as usize]);
                            addresses_final[i as usize] = addresses_absolute[i as usize];
                        }
                        AddressingMode::AbsoluteReg => {
                            memory.write(3 * i, u8::from($instr_name));
                            memory.write(3 * i + 1, addresses_absolute[i as usize]);
                            memory
                                .write(addresses_absolute_final_x[i as usize], values[i as usize]);
                            addresses_final[i as usize] = addresses_absolute_final_x[i as usize];
                        }
                        _ => panic!("Unsupported addressing mode {:?}", $addr_mode),
                    }
                }

                for i in 0..3 {
                    let value: (u8, bool) = values_res[i];
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);

                    if *$addr_mode == AddressingMode::Implied {
                        cpu.a = values[i];
                        cpu_copy.a = value.0;
                    }

                    if *$addr_mode == AddressingMode::ZeroPageReg || 
                       *$addr_mode == AddressingMode::AbsoluteReg {
                        cpu.x = x_values[i];
                        cpu_copy.x = x_values[i];
                    }

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.pc = pc + pc_increments[$addr_mode];
                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode];
                    cpu_copy.set_zero(value.0 == 0);
                    cpu_copy.set_negative((value.0 as i8) < 0);
                    cpu_copy.set_carry(value.1);

                    let res: u8 = memory.read(addresses_final[i as usize]);
                    assert_cpu(&cpu, &cpu_copy);
                    if *$addr_mode != AddressingMode::Implied {
                        assert_eq!(res, value.0);
                    }
                }
            }
        }
    }

    test_shifts! {test_asl_accumulator, Instruction::ASL_A, asl_func, &AddressingMode::Implied}
    test_shifts! {test_asl_zero_page, Instruction::ASL_ZP, asl_func, &AddressingMode::ZeroPage}
    test_shifts! {test_asl_zero_page_x, Instruction::ASL_ZP_X, asl_func, &AddressingMode::ZeroPageReg}
    test_shifts! {test_asl_absolute, Instruction::ASL_ABS, asl_func, &AddressingMode::Absolute}
    test_shifts! {test_asl_absolute_x, Instruction::ASL_ABS_X, asl_func, &AddressingMode::AbsoluteReg}

    test_shifts! {test_lsr_accumulator, Instruction::LSR_A, lsr_func, &AddressingMode::Implied}
    test_shifts! {test_lsr_zero_page, Instruction::LSR_ZP, lsr_func, &AddressingMode::ZeroPage}
    test_shifts! {test_lsr_zero_page_x, Instruction::LSR_ZP_X, lsr_func, &AddressingMode::ZeroPageReg}
    test_shifts! {test_lsr_absolute, Instruction::LSR_ABS, lsr_func, &AddressingMode::Absolute}
    test_shifts! {test_lsr_absolute_x, Instruction::LSR_ABS_X, lsr_func, &AddressingMode::AbsoluteReg}

    test_shifts! {test_rol_accumulator, Instruction::ROL_A, rol_func, &AddressingMode::Implied}
    test_shifts! {test_rol_zero_page, Instruction::ROL_ZP, rol_func, &AddressingMode::ZeroPage}
    test_shifts! {test_rol_zero_page_x, Instruction::ROL_ZP_X, rol_func, &AddressingMode::ZeroPageReg}
    test_shifts! {test_rol_absolute, Instruction::ROL_ABS, rol_func, &AddressingMode::Absolute}
    test_shifts! {test_rol_absolute_x, Instruction::ROL_ABS_X, rol_func, &AddressingMode::AbsoluteReg}

    test_shifts! {test_ror_accumulator, Instruction::ROR_A, ror_func, &AddressingMode::Implied}
    test_shifts! {test_ror_zero_page, Instruction::ROR_ZP, ror_func, &AddressingMode::ZeroPage}
    test_shifts! {test_ror_zero_page_x, Instruction::ROR_ZP_X, ror_func, &AddressingMode::ZeroPageReg}
    test_shifts! {test_ror_absolute, Instruction::ROR_ABS, ror_func, &AddressingMode::Absolute}
    test_shifts! {test_ror_absolute_x, Instruction::ROR_ABS_X, ror_func, &AddressingMode::AbsoluteReg}

    macro_rules! test_jmp {
        ($func_name: ident, $instr_name: expr, $addr_mode: expr) => {
            #[test]
            fn $func_name() {
                use std::collections::HashMap;
                if (*$addr_mode != AddressingMode::Indirect && *$addr_mode != AddressingMode::Absolute) {
                    panic!("Unsupported addressing mode {:?}", $addr_mode);
                }

                let mut cpu = CPU {
                    ..Default::default()
                };
        
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let mut cycles_increments: HashMap<AddressingMode, u64> = HashMap::new();
                cycles_increments.insert(AddressingMode::Absolute, 3);
                cycles_increments.insert(AddressingMode::Indirect, 5);

                let addresses = [0x1234, 0x4321, 0xABCD, 0x6942];
                let addresses_indirect = [0x2233, 0x3454, 0x6547, 0xBE43];
                let mut addresses_final = [0, 0, 0, 0];

                for i in 0..3 {
                    memory.write(3*i, u8::from($instr_name));
                    memory.write(3*i + 1, addresses[i as usize]);
                    addresses_final[i as usize] = addresses[i as usize];
                    if *$addr_mode == AddressingMode::Indirect {
                        memory.write(addresses[i as usize], addresses_indirect[i as usize]);
                        addresses_final[i as usize] = addresses_indirect[i as usize];
                    }
                }

                for i in 0..3 {
                    cpu.pc = 3*i;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.execute(&mut memory, instruction);

                    cpu_copy.pc = addresses_final[i as usize];
                    cpu_copy.cycles = cycles + cycles_increments[$addr_mode];

                    assert_cpu(&cpu, &cpu_copy);
                }
            }
        };
    }

    test_jmp! {test_jmp_absolute, Instruction::JMP_ABS, &AddressingMode::Absolute}
    test_jmp! {test_jmp_indirect, Instruction::JMP_IN, &AddressingMode::Indirect}

    #[test]
    fn test_jsr() {
        let mut cpu = CPU {
            ..Default::default()
        };

        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        cpu.sp = 0xff;
        let mut cpu_copy = cpu.clone();

        let addresses = [0x1234, 0x4321, 0xABCD, 0x6942];

        for i in 0..3 {
            memory.write(i*3, u8::from(Instruction::JSR_ABS));
            memory.write(i*3 + 1, addresses[i as usize]);
        }

        for i in 0..3 {
            cpu.pc = i * 3;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&memory);

            cpu.execute(&mut memory, instruction);

            cpu_copy.pc = addresses[i as usize];
            cpu_copy.cycles = cycles + 6;
            cpu_copy.sp -= 2;

            assert_cpu(&cpu, &cpu_copy);
            let stack_addr: u16 = memory.read(0x100u16 + cpu_copy.sp as u16 + 2);
            assert_eq!(stack_addr, i as u16 * 3 + 3 - 1, "Address on stack should be {}, but found {}", i as u16 * 3 + 3 - 1, stack_addr);
        }
    }

    #[test]
    fn test_rts() {
        let mut cpu = CPU {
            ..Default::default()
        };

        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        cpu.sp = 0xff;
        let mut cpu_copy = cpu.clone();

        let addresses = [0x1234, 0x4321, 0xABCD, 0x6942];

        for i in 0..3 {
            memory.write(i*3, u8::from(Instruction::JSR_ABS));
            memory.write(i*3 + 1, addresses[i as usize]);
            memory.write(addresses[i as usize], u8::from(Instruction::RTS_IM));
        }

        for i in 0..3 {
            let cycles = cpu.cycles;
            let instruction_jsr = cpu.fetch_instruction(&memory);
            
            cpu.execute(&mut memory, instruction_jsr);
            let instruction_rts = cpu.fetch_instruction(&memory);
            cpu.execute(&mut memory, instruction_rts);

            cpu_copy.pc = 3 * i + 3;
            cpu_copy.cycles = cycles + 12;

            assert_cpu(&cpu, &cpu_copy);
        }
    }

    #[derive(PartialEq, Eq)]
    enum FlagState {
        Set,
        Clear,
    }

    macro_rules! test_branches {
        ($func_name: ident, $instr_name: expr, $flag_state: expr) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };

                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();
                let mut cpu_copy = cpu.clone();

                let branch_addresses = [0xff, 0x10, 0xAB, 0x98];
                let mut additional_cycles = [0, 0, 0, 0];

                for i in 0..4 {
                    memory.write(2*i, u8::from($instr_name));
                    memory.write(2*i + 1, branch_addresses[i as usize]);

                    additional_cycles[i as usize] = ((2 * i + 1 + branch_addresses[i as usize]) > 0xff) as u64;
                }

                for i in 0..8 {
                    cpu.pc = 2 * (i / 2);
                    let instruction = cpu.fetch_instruction(&memory);
                    let pc = cpu.pc;

                    cpu.set_carry(i % 2 == 1);
                    cpu.set_negative(i % 2 == 1);
                    cpu.set_zero(i % 2 == 1);
                    cpu.set_overflow(i % 2 == 1);

                    cpu.execute(&mut memory, instruction);

                    let flag_set = match $flag_state {
                        FlagState::Set => 1,
                        FlagState::Clear => 0,
                    };

                    cpu_copy.pc = if i % 2 == flag_set {
                        pc + branch_addresses[(i / 2) as usize]
                    } else {
                        pc + 1
                    };

                    cpu_copy.cycles += if i % 2 == flag_set {
                        3 + additional_cycles[(i / 2) as usize]
                    } else {
                        2
                    };

                    cpu_copy.set_carry(i % 2 == 1);
                    cpu_copy.set_negative(i % 2 == 1);
                    cpu_copy.set_zero(i % 2 == 1);
                    cpu_copy.set_overflow(i % 2 == 1);

                    assert_cpu(&cpu, &cpu_copy);
                }

            }
        };
    }

    test_branches! {test_bcc, Instruction::BCC, FlagState::Clear}
    test_branches! {test_bcs, Instruction::BCS, FlagState::Set}
    test_branches! {test_beq, Instruction::BEQ, FlagState::Set}
    test_branches! {test_bmi, Instruction::BMI, FlagState::Set}
    test_branches! {test_bne, Instruction::BNE, FlagState::Clear}
    test_branches! {test_bpl, Instruction::BPL, FlagState::Clear}
    test_branches! {test_bvc, Instruction::BVC, FlagState::Clear}
    test_branches! {test_bvs, Instruction::BVS, FlagState::Set}

    #[test]
    fn test_bit_zero_page() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values1 = [0b0000_0000u8, 0b1111_1111, 0b1000_1111];
        let values2 = [0b1111_1111u8, 0b0101_0101, 0b1011_0011];
        let zero_flags = [true, false, false];
        let overflow_flags = [true, true, false];
        let negative_flags = [true, false, true];
        let addresses = [0x10, 0xAB, 0xFF];

        for i in 0..3 {
            memory.write_byte(2 * i, Instruction::BIT_ZP.into());
            memory.write_byte(2 * i + 1, addresses[i as usize]);
            memory.write_byte(addresses[i as usize] as u16, values2[i as usize]);
        }

        let mut cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = values1[i];

            cpu.execute(&mut memory, instruction);

            cpu_copy.cycles = cycles + 3;
            cpu_copy.pc = pc + 2;
            cpu_copy.a = values1[i];
            cpu_copy.set_zero(zero_flags[i]);
            cpu_copy.set_overflow(overflow_flags[i]);
            cpu_copy.set_negative(negative_flags[i]);
            assert_cpu(&cpu, &cpu_copy);
        }
    }

    #[test]
    fn test_bit_absolute() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values1 = [0b0000_0000u8, 0b1111_1111, 0b1000_1111];
        let values2 = [0b1111_1111u8, 0b0101_0101, 0b1011_0011];
        let zero_flags = [true, false, false];
        let overflow_flags = [true, true, false];
        let negative_flags = [true, false, true];
        let addresses = [0x1234, 0x4321, 0xABCD];

        for i in 0..3 {
            memory.write_byte(3 * i, Instruction::BIT_ABS.into());
            memory.write_word(3 * i + 1, addresses[i as usize]);
            memory.write_byte(addresses[i as usize] as u16, values2[i as usize]);
        }

        let mut cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = values1[i];

            cpu.execute(&mut memory, instruction);

            cpu_copy.cycles = cycles + 4;
            cpu_copy.pc = pc + 3;
            cpu_copy.a = values1[i];
            cpu_copy.set_zero(zero_flags[i]);
            cpu_copy.set_overflow(overflow_flags[i]);
            cpu_copy.set_negative(negative_flags[i]);
            assert_cpu(&cpu, &cpu_copy);
        }
    }

    #[test]
    fn test_reset() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.reset();

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.status, 0);
    }

    #[test]
    fn test_flag_carry() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_carry(false);
        assert_eq!(cpu.get_carry(), false);
        cpu.set_carry(true);
        assert_eq!(cpu.get_carry(), true);
        cpu.set_carry(false);
        assert_eq!(cpu.get_carry(), false);
    }

    #[test]
    fn test_flag_zero() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_zero(false);
        assert_eq!(cpu.get_zero(), false);
        cpu.set_zero(true);
        assert_eq!(cpu.get_zero(), true);
        cpu.set_zero(false);
        assert_eq!(cpu.get_zero(), false);
    }

    #[test]
    fn test_flag_interrupt_disable() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_interrupt_disable(false);
        assert_eq!(cpu.get_interrupt_disable(), false);
        cpu.set_interrupt_disable(true);
        assert_eq!(cpu.get_interrupt_disable(), true);
        cpu.set_interrupt_disable(false);
        assert_eq!(cpu.get_interrupt_disable(), false);
    }

    #[test]
    fn test_flag_decimal_mode() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_decimal_mode(false);
        assert_eq!(cpu.get_decimal_mode(), false);
        cpu.set_decimal_mode(true);
        assert_eq!(cpu.get_decimal_mode(), true);
        cpu.set_decimal_mode(false);
        assert_eq!(cpu.get_decimal_mode(), false);
    }

    #[test]
    fn test_flag_break_command() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_break_command(false);
        assert_eq!(cpu.get_break_command(), false);
        cpu.set_break_command(true);
        assert_eq!(cpu.get_break_command(), true);
        cpu.set_break_command(false);
        assert_eq!(cpu.get_break_command(), false);
    }

    #[test]
    fn test_flag_overflow() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_overflow(false);
        assert_eq!(cpu.get_overflow(), false);
        cpu.set_overflow(true);
        assert_eq!(cpu.get_overflow(), true);
        cpu.set_overflow(false);
        assert_eq!(cpu.get_overflow(), false);
    }

    #[test]
    fn test_flag_negative() {
        let mut cpu = CPU {
            ..Default::default()
        };

        cpu.set_negative(false);
        assert_eq!(cpu.get_negative(), false);
        cpu.set_negative(true);
        assert_eq!(cpu.get_negative(), true);
        cpu.set_negative(false);
        assert_eq!(cpu.get_negative(), false);
    }

    #[test]
    fn test_fetch_instruction() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };
        memory.write_byte(0x0000, Instruction::LDA_IM.into());

        let cpu_copy = cpu.clone();

        let instruction = cpu.fetch_instruction(&memory);

        assert_eq!(cpu.pc, cpu_copy.pc + 1);
        assert_eq!(
            <Instruction as Into<u8>>::into(instruction),
            Instruction::LDA_IM.into()
        );
    }
}

fn main() {
    let mut cpu = CPU {
        ..Default::default()
    };
    let mut memory = Memory {
        ..Default::default()
    };
    cpu.reset();

    cpu.execute(&mut memory, Instruction::LDA_ZP);
}
