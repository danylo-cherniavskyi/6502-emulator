type Byte = u8;
type Word = u16;

fn add_mod_256(n1: u8, n2: u8) -> u8 {
    let sum = ((n1 as u16 + n2 as u16) % 256) as u8;
    return sum;
}

fn add_mod_65536(n1: u16, n2: u16) -> u16 {
    let sum = (n1 as u32 + n2 as u32) % (0xffff + 1);
    return sum as u16;
}

pub trait MemoryLike<T> {
    fn read(&self, addr: Word) -> T;
    fn write(&mut self, addr: Word, value: T);
    fn read_zero_page(&self, pc: &mut Word) -> T;
    fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> T;
    fn read_absolute(&self, pc: &mut Word) -> T;
    fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> T;
    fn read_indirect_x(&self, pc: &mut Word, x: Byte) -> T;
    fn read_indirect_y(&self, pc: &mut Word, y: Byte) -> T;
}

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    ram: [Byte; 0x10000],
}

impl MemoryLike<u8> for Memory {
    fn read(&self, addr: Word) -> u8 {
        return self.ram[addr as usize];
    }

    fn write(&mut self, addr: Word, value: u8) {
        self.ram[addr as usize] = value;
    }

    fn read_zero_page(&self, pc: &mut Word) -> u8 {
        todo!();
    }

    fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> u8 {
        todo!();
    }

    fn read_absolute(&self, pc: &mut Word) -> u8 {
        todo!();
    }

    fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> u8 {
        todo!();
    }

    fn read_indirect_x(&self, pc: &mut Word, x: Byte) -> u8 {
        todo!();
    }

    fn read_indirect_y(&self, pc: &mut Word, y: Byte) -> u8 {
        todo!()
    }
}

impl MemoryLike<u16> for Memory {
    fn read(&self, addr: Word) -> u16 {
        return ((self.ram[(addr + 1) as usize] as u16) << 8) | self.ram[addr as usize] as u16;
    }

    fn write(&mut self, addr: Word, value: u16) {
        self.write(addr + 0, (value & 0x00ff) as u8);
        self.write(addr + 1, ((value & 0xff00) >> 8) as u8)
    }

    fn read_zero_page(&self, pc: &mut Word) -> u16 {
        todo!();
    }

    fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> u16 {
        todo!();
    }

    fn read_absolute(&self, pc: &mut Word) -> u16 {
        todo!();
    }

    fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> u16 {
        todo!();
    }

    fn read_indirect_x(&self, pc: &mut Word, x: Byte) -> u16 {
        todo!();
    }

    fn read_indirect_y(&self, pc: &mut Word, y: Byte) -> u16 {
        todo!();
    }
}

impl Memory {
    pub fn write_byte(&mut self, addr: Word, value: Byte) {
        self.ram[addr as usize] = value;
    }

    pub fn write_word(&mut self, addr: Word, value: Word) {
        self.write_byte(addr + 0, (value & 0x00ff) as u8);
        self.write_byte(addr + 1, ((value & 0xff00) >> 8) as u8)
    }

    pub fn read_immediate(&self, pc: &mut Word) -> u8 {
        todo!();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            ram: [0u8; 0x10000],
        }
    }
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

macro_rules! ld_immediate {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let value: u8 = memory.read(self.pc);
            self.$reg_name = value;
            self.test_number(value);

            self.pc += 1;
            self.cycles += 2;
        }
    };
}

macro_rules! ld_zero_page {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let address: u8 = memory.read(self.pc);
            let value: u8 = memory.read(address as u16);
            self.$reg_name = value;
            self.test_number(value);

            self.pc += 1;
            self.cycles += 3
        }
    };
}

macro_rules! ld_zero_page_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let address: u8 = memory.read(self.pc);
            let address_final = add_mod_256(address, self.$addr_reg);
            let value: u8 = memory.read(address_final as u16);
            self.$reg_name = value;
            self.test_number(value);

            self.pc += 1;
            self.cycles += 4
        }
    };
}

macro_rules! ld_absolute {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let address: u16 = memory.read(self.pc);
            let value: u8 = memory.read(address);
            self.$reg_name = value;
            self.test_number(value);

            self.pc += 2;
            self.cycles += 4
        }
    };
}

macro_rules! ld_absolute_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let instruction_address: u16 = memory.read(self.pc);
            let reg_address = self.$addr_reg;
            let address = add_mod_65536(instruction_address, reg_address as u16);
            let value: u8 = memory.read(address);
            self.$reg_name = value;
            self.test_number(value);

            self.pc += 2;
            self.cycles += if address < 256 { 4 } else { 5 };
        }
    };
}

impl CPU {
    ld_immediate! {lda_immediate, a}

    ld_zero_page! {lda_zero_page, a}

    ld_zero_page_reg! {lda_zero_page_x, a, x}

    ld_absolute! {lda_absolute, a}

    ld_absolute_reg! {lda_absolute_x, a, x}

    ld_absolute_reg! {lda_absolute_y, a, y}

    fn lda_indirect_x(&mut self, memory: &Memory) {
        let instruction_address: u8 = memory.read(self.pc);
        let x_address = self.x;
        let address = add_mod_256(instruction_address, x_address);
        let actual_address: u16 = memory.read(address as u16);
        let value: u8 = memory.read(actual_address);
        self.a = value;
        self.test_number(value);

        self.pc += 1;
        self.cycles += 6;
    }

    fn lda_indirect_y(&mut self, memory: &Memory) {
        let instruction_address: u8 = memory.read(self.pc);
        let y_address = self.y;
        let address_zp: u16 = memory.read(instruction_address as u16);
        let actual_address = add_mod_65536(address_zp, y_address as u16);
        let value: u8 = memory.read(actual_address);
        self.a = value;
        self.test_number(value);

        self.pc += 1;
        self.cycles += if actual_address < 256 { 5 } else { 6 };
    }

    // Sets flags based on number passed
    fn test_number(&mut self, num: u8) {
        self.set_zero(num == 0);
        self.set_negative((num & 0b1000_0000) != 0);
    }
}

impl CPU {
    ld_immediate! {ldx_immediate, x}

    ld_zero_page! {ldx_zero_page, x}

    ld_zero_page_reg! {ldx_zero_page_y, x, y}

    ld_absolute! {ldx_absolute, x}

    ld_absolute_reg! {ldx_absolute_y, x, y}
}

impl CPU {
    ld_immediate! {ldy_immediate, y}

    ld_zero_page! {ldy_zero_page, y}

    ld_zero_page_reg! {ldy_zero_page_x, y, x}

    ld_absolute! {ldy_absolute, y}

    ld_absolute_reg! {ldy_absolute_x, y, x}
}

macro_rules! st_zero_page {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address: u8 = memory.read(self.pc);
            memory.write_byte(address as u16, self.$reg_name);

            self.pc += 1;
            self.cycles += 3
        }
    };
}

macro_rules! st_zero_page_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address: u8 = memory.read(self.pc);
            let address_actual = add_mod_256(address, self.$addr_reg);
            memory.write_byte(address_actual as u16, self.$reg_name);

            self.pc += 1;
            self.cycles += 4;
        }
    };
}

macro_rules! st_absolute {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address: u16 = memory.read(self.pc);
            memory.write_byte(address, self.$reg_name);

            self.pc += 2;
            self.cycles += 4
        }
    };
}

macro_rules! st_absolute_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address: u16 = memory.read(self.pc);
            let address_actual = add_mod_65536(address, self.$addr_reg as u16);
            memory.write_byte(address_actual, self.$reg_name);

            self.pc += 2;
            self.cycles += 5;
        }
    };
}

impl CPU {
    st_zero_page! {sta_zero_page, a}

    st_zero_page_reg! {sta_zero_page_x, a, x}

    st_absolute! {sta_absolute, a}

    st_absolute_reg! {sta_absolute_x, a, x}

    st_absolute_reg! {sta_absolute_y, a, y}

    fn sta_indirect_x(&mut self, memory: &mut Memory) {
        let address: u8 = memory.read(self.pc);
        let x_address = self.x;
        let sum_address = add_mod_256(address, x_address);
        let address_actual: u16 = memory.read(sum_address as u16);
        memory.write_byte(address_actual, self.a);

        self.pc += 1;
        self.cycles += 6;
    }

    fn sta_indirect_y(&mut self, memory: &mut Memory) {
        let address: u8 = memory.read(self.pc);
        let y_address = self.y;
        let zp_address: u16 = memory.read(address as u16);
        let address_actual = add_mod_65536(zp_address, y_address as u16);
        memory.write_byte(address_actual, self.a);

        self.pc += 1;
        self.cycles += 6;
    }
}

impl CPU {
    st_zero_page! {stx_zero_page, x}

    st_zero_page_reg! {stx_zero_page_y, x, y}

    st_absolute! {stx_absolute, x}
}

impl CPU {
    st_zero_page! {sty_zero_page, y}

    st_zero_page_reg! {sty_zero_page_x, y, x}

    st_absolute! {sty_absolute, y}
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

macro_rules! logic_immediate {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let value: u8 = memory.read(self.pc);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 1;
            self.cycles += 2;
        }
    };
}

macro_rules! logic_zero_page {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let value_addr_zp: u8 = memory.read(self.pc);
            let value: u8 = memory.read(value_addr_zp as u16);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 1;
            self.cycles += 3;
        }
    };
}

macro_rules! logic_zero_page_x {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let value_addr_zp: u8 = memory.read(self.pc);
            let addr_actual = add_mod_256(value_addr_zp, self.x);
            let value: u8 = memory.read(addr_actual as u16);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 1;
            self.cycles += 4;
        }
    };
}

macro_rules! logic_absolute {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let addr: u16 = memory.read(self.pc);
            let value: u8 = memory.read(addr);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 2;
            self.cycles += 4
        }
    };
}

macro_rules! logic_absolute_reg {
    ($func_name: ident, $op_func: expr, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let addr: u16 = memory.read(self.pc);
            let addr_actual = add_mod_65536(addr, self.$reg_name as u16);
            let value: u8 = memory.read(addr_actual);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 2;
            self.cycles += 4;

            if addr_actual >= 0x100 {
                self.cycles += 1;
            }
        }
    };
}

macro_rules! logic_indirect_x {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let addr: u8 = memory.read(self.pc);
            let addr_zp_actual = add_mod_256(addr, self.x);
            let addr_on_zp: u16 = memory.read(addr_zp_actual as u16);
            let value: u8 = memory.read(addr_on_zp);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 1;
            self.cycles += 6;
        }
    };
}

macro_rules! logic_indirect_y {
    ($func_name: ident, $op_func: expr) => {
        fn $func_name(&mut self, memory: &Memory) {
            let addr: u8 = memory.read(self.pc);
            let addr_on_zp: u16 = memory.read(addr as u16);
            let addr_actual = add_mod_65536(addr_on_zp, self.y as u16);
            let value: u8 = memory.read(addr_actual);
            let res = $op_func(self.a, value);

            self.a = res;

            self.test_number(self.a);

            self.pc += 1;
            self.cycles += 5;

            if addr_actual >= 0x100 {
                self.cycles += 1;
            }
        }
    };
}

impl CPU {
    logic_immediate! {and_immediate, |n1, n2| n1 & n2}

    logic_zero_page! {and_zero_page, |n1, n2| n1 & n2}

    logic_zero_page_x! {and_zero_page_x, |n1, n2| n1 & n2}

    logic_absolute! {and_absolute, |n1, n2| n1 & n2}

    logic_absolute_reg! {and_absolute_x, |n1, n2| n1 & n2, x}

    logic_absolute_reg! {and_absolute_y, |n1, n2| n1 & n2, y}

    logic_indirect_x! {and_indirect_x, |n1, n2| n1 & n2}

    logic_indirect_y! {and_indirect_y, |n1, n2| n1 & n2}

    logic_immediate! {eor_immediate, |n1, n2| n1 ^ n2}

    logic_zero_page! {eor_zero_page, |n1, n2| n1 ^ n2}

    logic_zero_page_x! {eor_zero_page_x, |n1, n2| n1 ^ n2}

    logic_absolute! {eor_absolute, |n1, n2| n1 ^ n2}

    logic_absolute_reg! {eor_absolute_x, |n1, n2| n1 ^ n2, x}

    logic_absolute_reg! {eor_absolute_y, |n1, n2| n1 ^ n2, y}

    logic_indirect_x! {eor_indirect_x, |n1, n2| n1 ^ n2}

    logic_indirect_y! {eor_indirect_y, |n1, n2| n1 ^ n2}

    logic_immediate! {ora_immediate, |n1, n2| n1 | n2}

    logic_zero_page! {ora_zero_page, |n1, n2| n1 | n2}

    logic_zero_page_x! {ora_zero_page_x, |n1, n2| n1 | n2}

    logic_absolute! {ora_absolute, |n1, n2| n1 | n2}

    logic_absolute_reg! {ora_absolute_x, |n1, n2| n1 | n2, x}

    logic_absolute_reg! {ora_absolute_y, |n1, n2| n1 | n2, y}

    logic_indirect_x! {ora_indirect_x, |n1, n2| n1 | n2}

    logic_indirect_y! {ora_indirect_y, |n1, n2| n1 | n2}

    fn bit_zero_page(&mut self, memory: &Memory) {
        let addr: u8 = memory.read(self.pc);
        let value: u8 = memory.read(addr as u16);

        self.set_zero((self.a & value) == 0);
        self.set_overflow((value & 0b0100_0000) == 0b0100_0000);
        self.set_negative((value & 0b1000_0000) == 0b1000_0000);

        self.pc += 1;
        self.cycles += 3;
    }

    fn bit_absolute(&mut self, memory: &Memory) {
        let addr: u16 = memory.read(self.pc);
        let value: u8 = memory.read(addr);

        self.set_zero((self.a & value) == 0);
        self.set_overflow((value & 0b0100_0000) == 0b0100_0000);
        self.set_negative((value & 0b1000_0000) == 0b1000_0000);

        self.pc += 2;
        self.cycles += 4;
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn test_read_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let addresses = [0x0000u16, 0xffff, 0x6969, 0xABCD];
        let values = [0u8, 0xff, 0x42, 0xAB];

        for i in 0..4 {
            memory.ram[addresses[i] as usize] = values[i];
        }

        for i in 0..4 {
            let value: u8 = memory.read(addresses[i]);
            assert_eq!(value, values[i]);
        }
    }

    #[test]
    fn test_read_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let addresses = [0x0000u16, 0xfffe, 0x6969, 0xABCD];
        let values = [0x0000u16, 0xffff, 0x0420, 0x1234];

        for i in 0..4 {
            memory.ram[(addresses[i]) as usize] = (values[i] & 0xff) as u8;
            memory.ram[(addresses[i] + 1) as usize] = ((values[i] >> 8) & 0xff) as u8;
        }

        for i in 0..4 {
            let value: u16 = memory.read(addresses[i]);
            assert_eq!(value, values[i]);
        }
    }


    #[test]
    fn test_write_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let addresses = [0x0000, 0xffff, 0xABCD, 0xFEDC];
        let values = [0x00u8, 0xff, 0x12, 0xAB];

        for i in 0..4 {
            memory.write(addresses[i], values[i]);
            assert_eq!(values[i], memory.read(addresses[i]));
        }
    }

    #[test]
    fn test_write_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let addresses = [0x0000u16, 0xfffe, 0xABCD, 0xFEDC];
        let values = [0x0000u16, 0xffff, 0x1234, 0x9876];

        for i in 0..4 {
            memory.write(addresses[i], values[i]);
            assert_eq!(values[i], memory.read(addresses[i]));
        }
    }

    #[test]
    fn test_read_immediate() {
        let mut memory = Memory {
            ..Default::default()
        };

        let pc1 = 0x0010u16;
        let pc2 = 0xfffeu16;
        let pc3 = 0x1234u16;
        let pc4 = 0xABCDu16;

        let mut pcs = [pc1, pc2, pc3, pc4];
        let addresses = [0x0000u16, 0xfffeu16, 0x1234u16, 0xABCDu16];
        let values = [0x00u8, 0xff, 0xAB, 0x12];

        for i in 0..4 {
            memory.ram[pcs[i] as usize] = values[i];
        }

        for i in 0..4 {
            let value = memory.read_immediate(&mut pcs[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], addresses[i] + 1);
        }

    }

    #[test]
    fn test_read_zero_page_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let pcs_init = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let addresses = [0x00u8, 0xff, 0x12, 0xab];
        let values = [0x12u8, 0x00, 0xff, 0x69];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses[i] as u16, values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_zero_page(&mut pcs[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_read_zero_page_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let pcs_init = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let addresses = [0x00u8, 0xfe, 0x12, 0xab];
        let values = [0x1234u16, 0x0000, 0xffff, 0x6969];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses[i] as u16, values[i]);
        }

        for i in 0..4 {
            let value: u16 = memory.read_zero_page(&mut pcs[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_read_zero_page_x_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffe, 0x1234, 0xabcd];
        let pcs_init = [0x0000u16, 0xfffe, 0x1234, 0xabcd];
        let addresses = [0x00u8, 0xff, 0x12, 0xab];
        let x_addresses = [0x05u8, 0x10, 0xff, 0x11];
        let addresses_final = [0x05u8, 0x0f, 0x11, 0xbc];
        let values = [0x12u8, 0x00, 0xff, 0x69];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses_final[i] as u16, values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_zero_page_x(&mut pcs[i], x_addresses[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_read_zero_page_x_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffe, 0x1234, 0xabcd];
        let pcs_init = [0x0000u16, 0xfffe, 0x1234, 0xabcd];
        let addresses = [0x00u8, 0xfe, 0x12, 0xab];
        let x_addresses = [0x05u8, 0x10, 0xff, 0x11];
        let addresses_final = [0x05u8, 0x0e, 0x11, 0xbc];
        let values = [0x1234u16, 0x0000, 0xffff, 0x6969];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses_final[i] as u16, values[i]);
        }

        for i in 0..4 {
            let value: u16 = memory.read_zero_page_x(&mut pcs[i], x_addresses[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_read_absolute_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses[i], values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_absolute(&mut pcs[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_read_absolute_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let values = [0xffffu16, 0x0020, 0xabcd, 0x3109];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses[i], values[i]);
        }

        for i in 0..4 {
            let value: u16 = memory.read_absolute(&mut pcs[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_read_absolute_x_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let x_addresses = [0x00u8, 0x05, 0x52, 0x42];
        let addresses_final = [0x0010, 0xfff2, 0x1286, 0x1276];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses_final[i], values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_absolute_x(&mut pcs[i], x_addresses[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_read_absolute_x_u16() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let x_addresses = [0x00u8, 0x05, 0x52, 0x42];
        let addresses_final = [0x0010, 0xfff2, 0x1286, 0x1276];
        let values = [0xffffu16, 0x0020, 0xabcd, 0x3109];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses_final[i], values[i]);
        }

        for i in 0..4 {
            let value: u16 = memory.read_absolute_x(&mut pcs[i], x_addresses[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_read_indirect_x_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x05u8, 0xff, 0x14, 0x54];
        let x_addresses = [0x10u8, 0x05, 0x52, 0x42];
        let addresses_sum = [0x15u8, 0x04, 0x66, 0x96];
        let addresses_on_zp = [0x1015u16, 0xABDC, 0x6342, 0x9999];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses_sum[i] as u16, addresses_on_zp[i]);
            memory.write(addresses_on_zp[i], values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_indirect_x(&mut pcs[i], x_addresses[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_read_indirect_y_u8() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x05u8, 0xff, 0x14, 0x54];
        let addresses_on_zp = [0x1015u16, 0xABDC, 0x6342, 0x9999];
        let y_addresses = [0x10u8, 0x05, 0x52, 0x42];
        let addresses_sum = [0x1025u16, 0xABE1, 0x6394, 0x99DB];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
            memory.write(addresses[i] as u16, addresses_on_zp[i]);
            memory.write(addresses_sum[i] as u16, values[i]);
        }

        for i in 0..4 {
            let value: u8 = memory.read_indirect_y(&mut pcs[i], y_addresses[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    macro_rules! test_ld_immediate {
        ($func_name:ident, $reg_name:ident, $instr_name:ident) => {
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

                let values = [0u8, 69, (!10u8 + 1)];

                for i in 0..3 {
                    memory.write_byte(i * 2, Instruction::$instr_name.into());
                    memory.write_byte(i * 2 + 1, values[i as usize]);
                }

                for value in values {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);
                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 2);
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

    macro_rules! test_ld_zero_page {
        ($func_name:ident, $reg_name:ident, $instr_name:ident) => {
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

                let values = [0u8, 13, (!105u8 + 1)];
                let addresses = [0x13, 0x5A, 0xff];

                for i in 0..3 {
                    memory.write_byte(2 * i + 0, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses[i as usize] as u16, values[i as usize]);
                }

                for value in values {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);
                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 3);
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

    macro_rules! test_ld_zero_page_reg {
        ($func_name:ident, $reg_name:ident, $instr_name:ident, $addr_reg:ident) => {
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

                let values = [0u8, 45, (!105u8 + 1)];
                let addresses = [0x32u8, 0xBF, 0xFF];
                let addr_reg_values = [0x57u8, 0x64, 0x10];

                let addresses_actual = [0x89, 0x23, 0x0f];

                for i in 0..3 {
                    memory.write_byte(2 * i + 0, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses_actual[i as usize], values[i as usize]);
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    cpu.$addr_reg = addr_reg_values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.$addr_reg, addr_reg_values[i]);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_ld_absolute {
        ($func_name:ident, $reg_name:ident, $instr_name:ident) => {
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

                let values = [0u8, 45, (!105u8 + 1)];
                let addresses = [0x1234u16, 0x4321, 0xfff0];

                for i in 0..3 {
                    memory.write_byte(3 * i + 0, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses[i as usize], values[i as usize]);
                }

                for value in values {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_ld_absolute_reg {
        ($func_name:ident, $reg_name:ident, $instr_name:ident, $addr_reg:ident) => {
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

                let values = [0u8, 45, (!105u8 + 1)];
                let addresses = [0x1234u16, 0x0010, 0xfff0];
                let addr_reg_addresses = [0xff, 0xAB, 0x00];
                let addresses_actual = [0x1333u16, 0x00BB, 0xfff0];
                let additional_cycles = [1, 0, 1];

                for i in 0..3 {
                    memory.write_byte(3 * i + 0, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses_actual[i as usize], values[i as usize])
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    cpu.$addr_reg = addr_reg_addresses[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.$reg_name, value);
                    assert_eq!(cpu.$addr_reg, addr_reg_addresses[i]);
                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 4 + additional_cycles[i]);
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

    macro_rules! test_st_zero_page {
        ($func_name: ident, $reg_name: ident, $instr_name: ident) => {
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
                let addresses = [0xff, 0x69, 0x10];

                for i in 0..3 {
                    memory.write_byte(2 * i + 0, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let address = addresses[i];
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(address as u16);
                    assert_eq!(actual_value, value);

                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 3);
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

    macro_rules! test_st_zero_page_reg {
        ($func_name: ident, $reg_name: ident, $instr_name: ident, $addr_reg: ident) => {
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
                let reg_addresses = [0x00, 0x34, 0xCD];
                let addresses = [0xff, 0x35, 0x10];
                let addresses_actual = [0xff, 0x69, 0xDD];

                for i in 0..3 {
                    memory.write_byte(2 * i + 0, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$addr_reg = reg_addresses[i];
                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(addresses_actual[i] as u16);
                    assert_eq!(actual_value, value);

                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_st_absolute {
        ($func_name: ident, $reg_name: ident, $instr_name: ident) => {
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
                let addresses = [0x1234, 0x4321, 0x6969];

                for i in 0..3 {
                    memory.write_byte(3 * i + 0, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let address = addresses[i];
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(address as u16);
                    assert_eq!(actual_value, value);

                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_st_absolute_reg {
        ($func_name: ident, $reg_name: ident, $instr_name: ident, $addr_reg: ident) => {
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
                let reg_addresses = [0x10, 0xAB, 0x00];
                let addresses = [0x1234, 0x4321, 0x6969];
                let addresses_actual = [0x1244, 0x43CC, 0x6969];

                for i in 0..3 {
                    memory.write_byte(3 * i + 0, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                }

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let address = addresses_actual[i];
                    let value = values[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.$reg_name = value;
                    cpu.$addr_reg = reg_addresses[i];

                    cpu.execute(&mut memory, instruction);

                    let actual_value: u8 = memory.read(address as u16);
                    assert_eq!(actual_value, value);
                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 5);
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

    // LDA

    test_ld_immediate! {test_lda_immediate, a, LDA_IM}

    test_ld_zero_page! {test_lda_zero_page, a, LDA_ZP}

    test_ld_zero_page_reg! {test_lda_zero_page_x, a, LDA_ZP_X, x}

    test_ld_absolute! {test_lda_absolute, a, LDA_ABS}

    test_ld_absolute_reg! {test_lda_absolute_x, a, LDA_ABS_X, x}

    test_ld_absolute_reg! {test_lda_absolute_y, a, LDA_ABS_Y, y}

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        let cpu_copy = cpu.clone();

        let values = [0u8, 23, (!105u8 + 1)];
        let value_addresses = [0x1234, 0x4321, 0xABCD];
        let x_addresses = [0x10, 0x35, 0xAB];
        let addresses = [0x62, 0x34, 0x10];
        let addresses_actual = [0x72, 0x69, 0xBB];

        for i in 0..3 {
            memory.write_byte(i * 2 + 0, Instruction::LDA_IN_X.into());
            memory.write_byte(i * 2 + 1, addresses[i as usize]);

            memory.write_byte(value_addresses[i as usize], values[i as usize]);
            memory.write_word(addresses_actual[i as usize], value_addresses[i as usize]);
        }

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.x = x_addresses[i];
            let instruction = cpu.fetch_instruction(&memory);

            cpu.execute(&mut memory, instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.x, x_addresses[i]);
            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 6);
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

    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        let cpu_copy = cpu.clone();

        let values = [0u8, 23, (!105u8 + 1)];
        let address_addresses = [0x62, 0x34, 0x10];
        let addresses = [0x1224, 0x0034, 0xAB22];
        let y_addresses = [0x10, 0x35, 0xAB];
        let value_addresses = [0x1234, 0x0069, 0xABCD];
        let additional_cycles = [1, 0, 1];

        for i in 0..3 {
            memory.write_byte(i * 2 + 0, Instruction::LDA_IN_Y.into());
            memory.write_byte(i * 2 + 1, address_addresses[i as usize]);

            memory.write_byte(value_addresses[i as usize], values[i as usize]);
            memory.write_word(address_addresses[i as usize] as u16, addresses[i as usize]);
        }

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.y = y_addresses[i];
            let instruction = cpu.fetch_instruction(&memory);

            cpu.execute(&mut memory, instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.y, y_addresses[i]);
            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 5 + additional_cycles[i]);
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

    // LDX

    test_ld_immediate! {test_ldx_immediate, x, LDX_IM}

    test_ld_zero_page! {test_ldx_zero_page, x, LDX_ZP}

    test_ld_zero_page_reg! {test_ldx_zero_page_y, x, LDX_ZP_Y, y}

    test_ld_absolute! {test_ldx_absolute, x, LDX_ABS}

    test_ld_absolute_reg! {test_ldx_absolute_y, x, LDX_ABS_Y, y}

    // LDY

    test_ld_immediate! {test_ldy_immediate, y, LDY_IM}

    test_ld_zero_page! {test_ldy_zero_page, y, LDY_ZP}

    test_ld_zero_page_reg! {test_ldy_zero_page_x, y, LDY_ZP_X, x}

    test_ld_absolute! {test_ldy_absolute, y, LDY_ABS}

    test_ld_absolute_reg! {test_ldy_absolute_x, y, LDY_ABS_X, x}

    // STA
    test_st_zero_page! {test_sta_zero_page, a, STA_ZP}

    test_st_zero_page_reg! {test_sta_zero_page_x, a, STA_ZP_X, x}

    test_st_absolute! {test_sta_absolute, a, STA_ABS}

    test_st_absolute_reg! {test_sta_absolute_x, a, STA_ABS_X, x}

    test_st_absolute_reg! {test_sta_absolute_y, a, STA_ABS_Y, y}

    #[test]
    fn test_sta_indirect_x() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        let cpu_copy = cpu.clone();

        let values = [0u8, 69, (!105u8 + 1)];
        let x_addresses = [0x10, 0xAB, 0xF0];
        let instr_addresses = [0x40, 0x10, 0xF0];
        let zp_addresses = [0x50, 0xBB, 0xE0];
        let addresses_actual = [0x1244, 0x43CC, 0x6969];

        for i in 0..3 {
            memory.write_byte(2 * i + 0, Instruction::STA_IN_X.into());
            memory.write_byte(2 * i + 1, instr_addresses[i as usize]);
            memory.write_word(zp_addresses[i as usize], addresses_actual[i as usize]);
        }

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let address = addresses_actual[i];
            let value = values[i];
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = value;
            cpu.x = x_addresses[i];

            cpu.execute(&mut memory, instruction);

            let actual_value: u8 = memory.read(address as u16);
            assert_eq!(actual_value, value);
            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 6);
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

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut memory = Memory {
            ..Default::default()
        };

        cpu.reset();
        let cpu_copy = cpu.clone();

        let values = [0u8, 69, (!105u8 + 1)];
        let instr_addresses = [0x40, 0x10, 0xF0];
        let y_addresses = [0x10, 0xAB, 0xF0];
        let zp_addresses = [0x1234, 0x4321, 0x6879];
        let addresses_actual = [0x1244, 0x43CC, 0x6969];

        for i in 0..3 {
            memory.write_byte(2 * i + 0, Instruction::STA_IN_Y.into());
            memory.write_byte(2 * i + 1, instr_addresses[i as usize]);
            memory.write_word(instr_addresses[i as usize] as u16, zp_addresses[i as usize]);
        }

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let address = addresses_actual[i];
            let value = values[i];
            let instruction = cpu.fetch_instruction(&memory);

            cpu.a = value;
            cpu.y = y_addresses[i];

            cpu.execute(&mut memory, instruction);

            let actual_value: u8 = memory.read(address);
            assert_eq!(actual_value, value);
            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 6);
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

    // STX
    test_st_zero_page! {test_stx_zero_page, x, STX_ZP}

    test_st_zero_page_reg! {test_stx_zero_page_y, x, STX_ZP_Y, y}

    test_st_absolute! {test_stx_absolute, x, STX_ABS}

    // STY
    test_st_zero_page! {test_sty_zero_page, y, STY_ZP}

    test_st_zero_page_reg! {test_sty_zero_page_x, y, STY_ZP_X, x}

    test_st_absolute! {test_sty_absolute, y, STY_ABS}

    // Transfer
    macro_rules! test_transfer_reg_reg {
        ($func_name: ident, $reg_src: ident, $reg_dest: ident, $instr_name: ident, $test_flags_en: expr) => {
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
                    memory.write_byte(i, Instruction::$instr_name.into());
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

    test_transfer_reg_reg! {test_transfer_a_x, a, x, TAX, true}

    test_transfer_reg_reg! {test_transfer_a_y, a, y, TAY, true}

    test_transfer_reg_reg! {test_transfer_x_a, x, a, TXA, true}

    test_transfer_reg_reg! {test_transfer_y_a, y, a, TYA, true}

    test_transfer_reg_reg! {test_transfer_s_x, sp, x, TSX, true}

    test_transfer_reg_reg! {test_transfer_x_s, x, sp, TXS, false}

    // Stack
    macro_rules! test_push_stack {
        ($func_name: ident, $reg_name: ident, $instr_name: ident) => {
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
                    memory.write_byte(i, Instruction::$instr_name.into());
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
        ($func_name: ident, $reg_name: ident, $instr_name: ident, $instr_push_name: ident) => {
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
                    memory.write_byte(i, Instruction::$instr_name.into());
                    cpu.$reg_name = values[i as usize];
                    cpu.execute(&mut memory, Instruction::$instr_push_name.into());
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

    test_push_stack! {test_push_accumulator, a, PHA}

    test_push_stack! {test_push_processor_status, status, PHP}
    use Instruction::PLA;
    use Instruction::PLP;
    test_pull_stack! {test_pull_accumulator, a, PLA, PHA}

    test_pull_stack! {test_pull_processor_status, status, PLP, PHP}

    // Logical

    macro_rules! test_logic_immediate {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();

                for i in 0..3 {
                    memory.write_byte(2 * i, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 2);
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

    macro_rules! test_logic_zero_page {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses = [0x10, 0xAB, 0xFF];

                for i in 0..3 {
                    memory.write_byte(2 * i, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses[i as usize] as u16, values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 3);
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

    macro_rules! test_logic_zero_page_x {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses = [0x10, 0xAB, 0xFF];
                let x_addresses = [0x20, 0x10, 0x40];
                let addresses_actual = [0x30, 0xBB, 0x3F];

                for i in 0..3 {
                    memory.write_byte(2 * i, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses_actual[i as usize] as u16, values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];
                    cpu.x = x_addresses[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_logic_absolute {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses = [0x1234, 0x4321, 0xFF24];

                for i in 0..3 {
                    memory.write_byte(3 * i, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses[i as usize], values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 4);
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

    macro_rules! test_logic_absolute_reg {
        ($func_name: ident, $op_func: expr, $instr_name: ident, $reg_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses = [0x1234, 0x0020, 0xFF24];
                let reg_addresses = [0x21, 0x12, 0x69];
                let addresses_actual = [0x1255, 0x0032, 0xFF8D];

                let additional_cycles = [1, 0, 1];

                for i in 0..3 {
                    memory.write_byte(3 * i, Instruction::$instr_name.into());
                    memory.write_word(3 * i + 1, addresses[i as usize]);
                    memory.write_byte(addresses_actual[i as usize], values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];
                    cpu.$reg_name = reg_addresses[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 3);
                    assert_eq!(cpu.cycles, cycles + 4 + additional_cycles[i]);
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

    macro_rules! test_logic_indirect_x {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses = [0x10, 0x24, 0x70];
                let x_addresses = [0x20, 0x35, 0xFF];
                let addresses_zero_page_actual = [0x30, 0x59, 0x6F];
                let addresses_actual = [0x1234, 0x4321, 0xFF24];

                for i in 0..3 {
                    memory.write_byte(2 * i, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses[i as usize]);
                    memory.write_word(
                        addresses_zero_page_actual[i as usize],
                        addresses_actual[i as usize],
                    );
                    memory.write_byte(addresses_actual[i as usize], values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.a = values1[i];
                    cpu.x = x_addresses[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 6);
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

    macro_rules! test_logic_indirect_y {
        ($func_name: ident, $op_func: expr, $instr_name: ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory {
                    ..Default::default()
                };

                cpu.reset();

                let values1 = [0b0000_0000u8, 0b1111_1111, 0b0000_1111];
                let values2 = [0b1111_1111u8, 0b0101_0101, 0b0011_0011];
                let values_res: Vec<u8> = zip(values1, values2)
                    .map(|pair| $op_func(pair.0, pair.1))
                    .collect();
                let addresses_zp = [0x10, 0x23, 0x96];
                let addresses_in_zp = [0x1234, 0x0045, 0xABCD];
                let y_addresses = [0x54, 0x24, 0xAB];
                let addresses_actual = [0x1288, 0x0069, 0xAC78];

                let additional_cycles = [1, 0, 1];

                for i in 0..3 {
                    memory.write_byte(2 * i, Instruction::$instr_name.into());
                    memory.write_byte(2 * i + 1, addresses_zp[i as usize]);
                    memory.write_word(addresses_zp[i as usize] as u16, addresses_in_zp[i as usize]);
                    memory.write_byte(addresses_actual[i as usize], values2[i as usize]);
                }

                let cpu_copy = cpu.clone();

                for i in 0..3 {
                    let pc = cpu.pc;
                    let cycles = cpu.cycles;
                    let value = values_res[i];
                    let instruction = cpu.fetch_instruction(&memory);

                    cpu.y = y_addresses[i];
                    cpu.a = values1[i];

                    cpu.execute(&mut memory, instruction);

                    assert_eq!(cpu.a, value);
                    assert_eq!(cpu.pc, pc + 2);
                    assert_eq!(cpu.cycles, cycles + 5 + additional_cycles[i]);
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

    test_logic_immediate! {test_and_immediate, |n1, n2| n1 & n2, AND_IM}

    test_logic_immediate! {test_eor_immediate, |n1, n2| n1 ^ n2, EOR_IM}

    test_logic_immediate! {test_ora_immediate, |n1, n2| n1 | n2, ORA_IM}

    test_logic_zero_page! {test_and_zero_page, |n1, n2| n1 & n2, AND_ZP}

    test_logic_zero_page! {test_eor_zero_page, |n1, n2| n1 ^ n2, EOR_ZP}

    test_logic_zero_page! {test_ora_zero_page, |n1, n2| n1 | n2, ORA_ZP}

    test_logic_zero_page_x! {test_and_zero_page_x, |n1, n2| n1 & n2, AND_ZP_X}

    test_logic_zero_page_x! {test_eor_zero_page_x, |n1, n2| n1 ^ n2, EOR_ZP_X}

    test_logic_zero_page_x! {test_ora_zero_page_x, |n1, n2| n1 | n2, ORA_ZP_X}

    test_logic_absolute! {test_and_absolute, |n1, n2| n1 & n2, AND_ABS}

    test_logic_absolute! {test_eor_absolute, |n1, n2| n1 ^ n2, EOR_ABS}

    test_logic_absolute! {test_ora_absolute, |n1, n2| n1 | n2, ORA_ABS}

    test_logic_absolute_reg! {test_and_absolute_x, |n1, n2| n1 & n2, AND_ABS_X, x}

    test_logic_absolute_reg! {test_eor_absolute_x, |n1, n2| n1 ^ n2, EOR_ABS_X, x}

    test_logic_absolute_reg! {test_ora_absolute_x, |n1, n2| n1 | n2, ORA_ABS_X, x}

    test_logic_absolute_reg! {test_and_absolute_y, |n1, n2| n1 & n2, AND_ABS_Y, y}

    test_logic_absolute_reg! {test_eor_absolute_y, |n1, n2| n1 ^ n2, EOR_ABS_Y, y}

    test_logic_absolute_reg! {test_ora_absolute_y, |n1, n2| n1 | n2, ORA_ABS_Y, y}

    test_logic_indirect_x! {test_and_indirect_x, |n1, n2| n1 & n2, AND_IN_X}

    test_logic_indirect_x! {test_eor_indirect_x, |n1, n2| n1 ^ n2, EOR_IN_X}

    test_logic_indirect_x! {test_ora_indirect_x, |n1, n2| n1 | n2, ORA_IN_X}

    test_logic_indirect_y! {test_and_indirect_y, |n1, n2| n1 & n2, AND_IN_Y}

    test_logic_indirect_y! {test_eor_indirect_y, |n1, n2| n1 ^ n2, EOR_IN_Y}

    test_logic_indirect_y! {test_ora_indirect_y, |n1, n2| n1 | n2, ORA_IN_Y}

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
