pub mod memory;

use memory::memory::{AddressingMode, Memory, MemoryLike, Byte, Word};

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
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(&mut self.pc, self.$addr_reg, &mut page_crossed),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed),
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
                AddressingMode::ZeroPageReg => memory.write_zero_page_x(&mut self.pc, self.$addr_reg, self.$reg_name),
                AddressingMode::Absolute => memory.write_absolute(&mut self.pc, self.$reg_name),
                AddressingMode::AbsoluteReg => memory.write_absolute_x(&mut self.pc, self.$addr_reg, self.$reg_name),
                AddressingMode::IndirectX => memory.write_indirect_x(&mut self.pc, self.x, self.$reg_name),
                AddressingMode::IndirectY => memory.write_indirect_y(&mut self.pc, self.y, self.$reg_name),
                _ => panic!("Unsupported addressing mode {:?}", $addr_mode)
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

            self.sp += 1;
            self.cycles += 3;
        }
    };
}

macro_rules! pull_reg {
    ($func_name: ident, $reg_name: ident, $test_en: expr) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            self.sp -= 1;
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
                AddressingMode::AbsoluteReg => memory.read_absolute_x_check_crossing(&mut self.pc, self.$reg_name, &mut page_crossed),
                AddressingMode::IndirectX => memory.read_indirect_x(&mut self.pc, self.x),
                AddressingMode::IndirectY => memory.read_indirect_y_check_crossing(&mut self.pc, self.y, &mut page_crossed),
            };

            self.a = $op_func(value1, value2);
            self.test_number(self.a);

            self.cycles += cycles[$addr_mode] + if page_crossed {1} else {0};
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

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

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
                let cpu_copy = cpu.clone();

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

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.pc, pc + pc_increments[$addr_mode]);
                    assert_eq!(
                        cpu.cycles,
                        cycles + cycles_increments[$addr_mode] + additional_cycles[i]
                    );
                    assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
                    assert_eq!(cpu.get_zero(), value == 0);
                    assert_eq!(
                        cpu.get_interrupt_disable(),
                        cpu_copy.get_interrupt_disable()
                    );
                    assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
                    assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
                    assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
                    assert_eq!(cpu.get_negative(), (value as i8) < 0);
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
                let cpu_copy = cpu.clone();

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

                    assert_eq!(cpu.pc, pc + pc_increments[$addr_mode]);
                    assert_eq!(cpu.cycles, cycles + cycles_increments[$addr_mode]);
                    assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
                    assert_eq!(cpu.get_zero(), cpu_copy.get_zero());
                    assert_eq!(
                        cpu.get_interrupt_disable(),
                        cpu_copy.get_interrupt_disable()
                    );
                    assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
                    assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
                    assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
                    assert_eq!(cpu.get_negative(), cpu_copy.get_negative());
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
                let cpu_copy = cpu.clone();

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

                    assert_eq!(cpu.$reg_dest, value);
                    assert_eq!(cpu.pc, pc + 1);
                    assert_eq!(cpu.cycles, cycles + 2);
                    assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
                    assert_eq!(
                        cpu.get_interrupt_disable(),
                        cpu_copy.get_interrupt_disable()
                    );
                    assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
                    assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
                    assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
                    if $test_flags_en {
                        assert_eq!(cpu.get_zero(), value == 0);
                        assert_eq!(cpu.get_negative(), (value as i8) < 0);
                    } else {
                        assert_eq!(cpu.get_zero(), cpu_copy.get_zero());
                        assert_eq!(cpu.get_negative(), cpu_copy.get_negative());
                    }
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

                    let actual_value: u8 = memory.read(0x0100 + (cpu.sp - 1) as u16);
                    assert_eq!(actual_value, value);
                    assert_eq!(cpu.sp, (i + 1) as u8);
                    assert_eq!(cpu.pc, pc + 1);
                    assert_eq!(cpu.cycles, cycles + 3);
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

                let values = [0u8, 69, (!105u8 + 1)];

                for i in 0..3 {
                    memory.write_byte(i, $instr_name.into());
                    cpu.$reg_name = values[i as usize];
                    cpu.execute(&mut memory, $instr_push_name.into());
                }
                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[values.len() - i - 1];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.sp, (cpu_copy.sp - i as u8 - 1) as u8);
                    assert_eq!(cpu.pc, pc + 1);
                    assert_eq!(cpu.cycles, cycles + 4);
                    if (u8::from(Instruction::PLA) == u8::from($instr_name)) {
                        assert_eq!(cpu.get_zero(), value == 0);
                        assert_eq!(cpu.get_negative(), (value as i8) < 0);
                    }
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
                let cpu_copy = cpu.clone();

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

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + pc_increments[$addr_mode]);
                    assert_eq!(
                        cpu.cycles,
                        cycles + cycles_increments[$addr_mode] + additional_cycles[i]
                    );
                    assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
                    assert_eq!(cpu.get_zero(), value == 0);
                    assert_eq!(
                        cpu.get_interrupt_disable(),
                        cpu_copy.get_interrupt_disable()
                    );
                    assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
                    assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
                    assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
                    assert_eq!(cpu.get_negative(), (value as i8) < 0);
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

        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = values1[i];

            cpu.execute(&mut memory, instruction);

            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 3);
            assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
            assert_eq!(cpu.get_zero(), zero_flags[i]);
            assert_eq!(
                cpu.get_interrupt_disable(),
                cpu_copy.get_interrupt_disable()
            );
            assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
            assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
            assert_eq!(cpu.get_overflow(), overflow_flags[i]);
            assert_eq!(cpu.get_negative(), negative_flags[i]);
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

        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = values1[i];

            cpu.execute(&mut memory, instruction);

            assert_eq!(cpu.pc, pc + 3);
            assert_eq!(cpu.cycles, cycles + 4);
            assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
            assert_eq!(cpu.get_zero(), zero_flags[i]);
            assert_eq!(
                cpu.get_interrupt_disable(),
                cpu_copy.get_interrupt_disable()
            );
            assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
            assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
            assert_eq!(cpu.get_overflow(), overflow_flags[i]);
            assert_eq!(cpu.get_negative(), negative_flags[i]);
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
