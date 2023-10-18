type Byte = u8;
type Word = u16;

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    ram: [Byte; 0xffff],
}

impl Memory {
    pub fn read_byte(&self, addr: Word) -> Byte {
        return self.ram[addr as usize];
    }

    pub fn read_word(&self, addr: Word) -> Word {
        return ((self.ram[(addr + 1) as usize] as u16) << 8) | self.ram[addr as usize] as u16;
    }

    pub fn write_byte(&mut self, addr: Word, value: Byte) {
        self.ram[addr as usize] = value;
    }

    pub fn write_word(&mut self, addr: Word, value: Word) {
        self.write_byte(addr + 0, (value & 0x00ff) as u8);
        self.write_byte(addr + 1, ((value & 0xff00) >> 8) as u8)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory { ram: [0u8; 0xffff] }
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

            Instruction::INVALID => println!("Error: Invalid instruction"),
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> Instruction {
        let instruction = memory.read_byte(self.pc);
        self.pc += 1;

        return Instruction::from(instruction);
    }
}

macro_rules! ld_immediate {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &Memory) {
            let value = memory.read_byte(self.pc);
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
            let address = memory.read_byte(self.pc);
            let value = memory.read_byte(address as u16);
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
            let address = memory.read_byte(self.pc);
            let address_final = self.add_mod_256(address, self.$addr_reg);
            let value = memory.read_byte(address_final as u16);
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
            let address = memory.read_word(self.pc);
            let value = memory.read_byte(address);
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
            let instruction_address = memory.read_word(self.pc);
            let reg_address = self.$addr_reg;
            let address = instruction_address + reg_address as u16;
            let value = memory.read_byte(address);
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
        let instruction_address = memory.read_byte(self.pc);
        let x_address = self.x;
        let address = self.add_mod_256(instruction_address, x_address);
        let actual_address = memory.read_word(address as u16);
        let value = memory.read_byte(actual_address);
        self.a = value;
        self.test_number(value);

        self.pc += 1;
        self.cycles += 6;
    }

    fn lda_indirect_y(&mut self, memory: &Memory) {
        let instruction_address = memory.read_byte(self.pc);
        let y_address = self.y;
        let address_zp = memory.read_word(instruction_address as u16);
        let actual_address = address_zp + y_address as u16;
        let value = memory.read_byte(actual_address);
        self.a = value;
        self.test_number(value);

        self.pc += 1;
        self.cycles += if actual_address < 256 { 5 } else { 6 };
    }

    fn add_mod_256(&mut self, n1: u8, n2: u8) -> u8 {
        let sum = ((n1 as u16 + n2 as u16) % 256) as u8;
        return sum;
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
            let address = memory.read_byte(self.pc);
            memory.write_byte(address as u16, self.$reg_name);

            self.pc += 1;
            self.cycles += 3
        }
    };
}

macro_rules! st_zero_page_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address = memory.read_byte(self.pc);
            let address_actual = self.add_mod_256(address, self.$addr_reg);
            memory.write_byte(address_actual as u16, self.$reg_name);

            self.pc += 1;
            self.cycles += 4;
        }
    };
}

macro_rules! st_absolute {
    ($func_name: ident, $reg_name: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address = memory.read_word(self.pc);
            memory.write_byte(address, self.$reg_name);

            self.pc += 2;
            self.cycles += 4
        }
    };
}

macro_rules! st_absolute_reg {
    ($func_name: ident, $reg_name: ident, $addr_reg: ident) => {
        fn $func_name(&mut self, memory: &mut Memory) {
            let address = memory.read_word(self.pc);
            // TODO: make addition wrap around
            let address_actual = address + self.$addr_reg as u16;
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
        todo!();
    }

    fn sta_indirect_y(&mut self, memory: &mut Memory) {
        todo!();
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ld_immediate {
        ($func_name:ident, $reg_name:ident, $instr_name:ident) => {
            #[test]
            fn $func_name() {
                let mut cpu = CPU {
                    ..Default::default()
                };
                let mut memory = Memory { ram: [0u8; 0xffff] };
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

                    assert_eq!(memory.read_byte(address as u16), value);

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

                    assert_eq!(memory.read_byte(addresses_actual[i] as u16), value);

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

                    assert_eq!(memory.read_byte(address as u16), value);

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

                    assert_eq!(memory.read_byte(address as u16), value);
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
        let mut memory = Memory { ram: [0u8; 0xffff] };
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

            assert_eq!(memory.read_byte(address as u16), value);
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
            memory.write_byte(2 * i + 0, Instruction::STA_IN_X.into());
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

            assert_eq!(memory.read_byte(address as u16), value);
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
}

fn main() {
    let mut cpu = CPU {
        ..Default::default()
    };
    let mut memory = Memory { ram: [0u8; 0xffff] };
    cpu.reset();

    cpu.execute(&mut memory, Instruction::LDA_ZP);
}
