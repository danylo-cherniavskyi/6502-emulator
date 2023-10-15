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
    LDA_IM,
    LDA_ZP,
    LDA_ZP_X,
    LDA_ABS,
    LDA_ABS_X,
    LDA_ABS_Y,
    LDA_IN_X,
    LDA_IN_Y,
    INVALID,
}

impl From<u8> for Instruction {
    fn from(a: u8) -> Self {
        match a {
            0xA9 => Instruction::LDA_IM,
            0xA5 => Instruction::LDA_ZP,
            0xB5 => Instruction::LDA_ZP_X,
            0xAD => Instruction::LDA_ABS,
            0xBD => Instruction::LDA_ABS_X,
            0xB9 => Instruction::LDA_ABS_Y,
            0xA1 => Instruction::LDA_IN_X,
            0xB1 => Instruction::LDA_IN_Y,
            _ => Instruction::INVALID,
        }
    }
}

impl From<Instruction> for u8 {
    fn from(a: Instruction) -> Self {
        match a {
            Instruction::LDA_IM => 0xA9,
            Instruction::LDA_ZP => 0xA5,
            Instruction::LDA_ZP_X => 0xB5,
            Instruction::LDA_ABS => 0xAD,
            Instruction::LDA_ABS_X => 0xBD,
            Instruction::LDA_ABS_Y => 0xB9,
            Instruction::LDA_IN_X => 0xA1,
            Instruction::LDA_IN_Y => 0xB1,
            Instruction::INVALID => 0xFF,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CPU<'a> {
    cycles: u64,
    pc: Word,
    sp: Byte,
    a: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
    memory: Option<&'a Memory>,
}

impl Default for CPU<'_> {
    fn default() -> Self {
        CPU {
            cycles: 0,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            memory: Option::None,
        }
    }
}

impl CPU<'_> {
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

    pub fn execute(&mut self, i: Instruction) {
        match i {
            Instruction::LDA_IM => self.lda_immediate(),
            Instruction::LDA_ZP => self.lda_zero_page(),
            Instruction::LDA_ZP_X => self.lda_zero_page_x(),
            Instruction::LDA_ABS => self.lda_absolute(),
            Instruction::LDA_ABS_X => self.lda_absolute_x(),
            Instruction::LDA_ABS_Y => self.lda_absolute_y(),
            Instruction::LDA_IN_X => self.lda_indirect_x(),
            Instruction::LDA_IN_Y => self.lda_indirect_y(),
            Instruction::INVALID => println!("Error: Invalid instruction"),
        }
    }

    pub fn fetch_instruction(&mut self) -> Instruction {
        let instruction = self.read_byte(self.pc);
        self.pc += 1;

        return Instruction::from(instruction);
    }

    fn read_byte(&mut self, addr: Word) -> Byte {
        return self.memory.unwrap().read_byte(addr);
    }

    fn read_word(&mut self, addr: Word) -> Word {
        return self.memory.unwrap().read_word(addr);
    }
}

impl CPU<'_> {
    fn lda_immediate(&mut self) {
        let value = self.read_byte(self.pc);
        self.a = value;

        self.set_zero(value == 0);
        self.set_negative((value & 0b1000_0000) != 0);
        self.pc += 1;
        self.cycles += 2
    }

    fn lda_zero_page(&mut self) {
        let address = self.read_byte(self.pc);
        let value = self.read_byte(address as u16);
        self.a = value;

        self.set_zero(value == 0);
        self.set_negative((value & 0b1000_0000) != 0);
        self.pc += 1;
        self.cycles += 3
    }

    fn lda_zero_page_x(&mut self) {
        let address = self.read_byte(self.pc);
        let address_final = self.add(address, self.x);
        let value = self.read_byte(address_final as u16);
        self.a = value;

        self.set_zero(value == 0);
        self.set_negative((value & 0b1000_0000) != 0);
        self.pc += 1;
        self.cycles += 4
    }

    fn lda_absolute(&mut self) {
        let address = self.read_word(self.pc);
        let value = self.read_byte(address);
        self.a = value;

        self.set_zero(value == 0);
        self.set_negative((value & 0b1000_0000) != 0);
        self.pc += 2;
        self.cycles += 4
    }

    fn lda_absolute_x(&mut self) {
        todo!();
    }

    fn lda_absolute_y(&mut self) {
        todo!();
    }

    fn lda_indirect_x(&mut self) {
        todo!();
    }

    fn lda_indirect_y(&mut self) {
        todo!();
    }

    fn add(&mut self, n1: u8, n2: u8) -> u8 {
        let sum = ((n1 as u16 + n2 as u16) % 256) as u8;
        self.cycles += 1;
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        cpu.memory = Some(&memory);
        let cpu_copy = cpu.clone();

        let instruction = cpu.fetch_instruction();

        assert_eq!(cpu.pc, cpu_copy.pc + 1);
        assert_eq!(
            <Instruction as Into<u8>>::into(instruction),
            Instruction::LDA_IM.into()
        );
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut mem = Memory { ram: [0u8; 0xffff] };
        cpu.reset();

        let values = [0u8, 69, (!10u8 + 1)];

        for i in 0..3 {
            mem.write_byte(i * 2, Instruction::LDA_IM.into());
            mem.write_byte(i * 2 + 1, values[i as usize]);
        }

        cpu.memory = Some(&mem);
        let cpu_copy = cpu.clone();

        for value in values {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction();
            cpu.execute(instruction);

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

    #[test]
    fn test_lda_zero_page() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut mem = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 13, (!105u8 + 1)];
        let addresses = [0x13, 0x5A, 0xff];

        for i in 0..3 {
            mem.write_byte(2 * i + 0, Instruction::LDA_ZP.into());
            mem.write_byte(2 * i + 1, addresses[i as usize]);
            mem.write_byte(addresses[i as usize] as u16, values[i as usize]);
        }

        cpu.memory = Some(&mem);
        let cpu_copy = cpu.clone();

        for value in values {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction();
            cpu.execute(instruction);

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

    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut m = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 45, (!105u8 + 1)];
        let addresses = [0x32u8, 0xBF, 0xFF];
        let x_values = [0x57u8, 0x64, 0x10];

        let addresses_actual = [0x89, 0x23, 0x0f];

        for i in 0..3 {
            m.write_byte(2 * i + 0, Instruction::LDA_ZP_X.into());
            m.write_byte(2 * i + 1, addresses[i as usize]);
            m.write_byte(addresses_actual[i as usize], values[i as usize]);
        }

        cpu.memory = Some(&m);
        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.x = x_values[i];
            let instruction = cpu.fetch_instruction();

            cpu.execute(instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.x, x_values[i]);
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

    #[test]
    fn test_lda_absolute() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut m = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 45, (!105u8 + 1)];
        let addresses = [0x1234u16, 0x4321, 0xfff0];

        for i in 0..3 {
            m.write_byte(3 * i + 0, Instruction::LDA_ABS.into());
            m.write_word(3 * i + 1, addresses[i as usize]);
            m.write_byte(addresses[i as usize], values[i as usize]);
        }

        cpu.memory = Some(&m);
        let cpu_copy = cpu.clone();

        for value in values {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction();

            cpu.execute(instruction);

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

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut m = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 45, (!105u8 + 1)];
        let addresses = [0x1234u16, 0x0010, 0xfff0];
        let x_addresses = [0xff, 0xAB, 0x00];
        let addresses_actual = [0x1333u16, 0x00BB, 0xfff0];
        let additional_cycles = [1, 0, 1];

        for i in 0..3 {
            m.write_byte(3 * i + 0, Instruction::LDA_ABS_X.into());
            m.write_word(3 * i + 1, addresses[i as usize]);
            m.write_byte(addresses_actual[i as usize], values[i as usize])
        }

        cpu.memory = Some(&m);
        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.x = x_addresses[i];
            let instruction = cpu.fetch_instruction();

            cpu.execute(instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.x, x_addresses[i]);
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

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut m = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 45, (!105u8 + 1)];
        let addresses = [0x1234u16, 0x0010, 0xfff0];
        let y_addresses = [0xff, 0xAB, 0x00];
        let addresses_actual = [0x1333u16, 0x00BB, 0xfff0];
        let additional_cycles = [1, 0, 1];

        for i in 0..3 {
            m.write_byte(3 * i + 0, Instruction::LDA_ABS_Y.into());
            m.write_word(3 * i + 1, addresses[i as usize]);
            m.write_byte(addresses_actual[i as usize], values[i as usize])
        }

        cpu.memory = Some(&m);
        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.y = y_addresses[i];
            let instruction = cpu.fetch_instruction();

            cpu.execute(instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.y, y_addresses[i]);
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

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = CPU {
            ..Default::default()
        };
        let mut m = Memory {
            ..Default::default()
        };

        cpu.reset();

        let values = [0u8, 23, (!105u8 + 1)];
        let value_addresses = [0x1234, 0x4321, 0xABCD];
        let x_addresses = [0x10, 0x35, 0xAB];
        let addresses = [0x62, 0x34, 0x10];
        let addresses_actual = [0x72, 0x69, 0xBB];

        for i in 0..3 {
            m.write_byte(i * 2 + 0, Instruction::LDA_IN_X.into());
            m.write_byte(i * 2 + 1, addresses[i as usize]);

            m.write_byte(value_addresses[i as usize], values[i as usize]);
            m.write_word(addresses_actual[i as usize], value_addresses[i as usize]);
        }

        cpu.memory = Some(&m);
        let cpu_copy = cpu.clone();

        for i in 0..3 {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let value = values[i];
            cpu.x = x_addresses[i];
            let instruction = cpu.fetch_instruction();

            cpu.execute(instruction);

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
        todo!()
    }
}

fn main() {
    let mut cpu = CPU {
        ..Default::default()
    };
    let memory = Memory { ram: [0u8; 0xffff] };
    cpu.memory = Some(&memory);
    cpu.reset();

    cpu.execute(Instruction::LDA_ZP);
}
