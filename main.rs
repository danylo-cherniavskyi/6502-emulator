type Byte = u8;
type Word = u16;

pub struct Memory {
    ram: [Byte; 0xffff]
}

impl Memory {
    fn read_byte(&self, addr: Word) -> Byte {
        return self.ram[addr as usize];
    }

    fn read_word(&self, addr: Word) -> Word {
        return ((self.ram[(addr + 1) as usize] as u16) << 8) | 
                 self.ram[ addr      as usize] as u16;
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
    INVALID
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
            _    => Instruction::INVALID,
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
            Instruction::INVALID => 0xFF
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
        return self.status
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

    pub fn execute(&mut self, m: &mut Memory, i: Instruction) {
        match i {
            Instruction::LDA_IM => self.lda_immediate(m),
            Instruction::LDA_ZP => self.lda_zero_page(m),
            Instruction::LDA_ZP_X => self.lda_zero_page_x(m),
            Instruction::LDA_ABS => self.lda_absolute(m),
            Instruction::LDA_ABS_X => self.lda_absolute_x(m),
            Instruction::LDA_ABS_Y => self.lda_absolute_y(m),
            Instruction::LDA_IN_X => self.lda_indirect_x(m),
            Instruction::LDA_IN_Y => self.lda_indirect_y(m),
            Instruction::INVALID => println!("Error: Invalid instruction"),
        }
    }

    pub fn fetch_instruction(&mut self, m: &Memory) -> Instruction {
        let instruction = m.ram[self.pc as usize];
        self.pc += 1;
        self.cycles += 1;

        return Instruction::from(instruction);
    }
}

impl CPU {
    fn lda_immediate(&mut self, m: &mut Memory) {
        let value = m.read_byte(self.pc);
        self.a = value;

        self.set_zero(value == 0);
        self.set_negative((value & 0b1000_0000) != 0);

        self.pc += 1;
        self.cycles += 1;
    }

    fn lda_zero_page(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_zero_page_x(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_zero_page_y(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_absolute(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_absolute_x(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_absolute_y(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_indirect_x(&mut self, m: &mut Memory) {
        todo!();
    }

    fn lda_indirect_y(&mut self, m: &mut Memory) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut cpu = CPU {..Default::default()};

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
        let mut cpu = CPU {..Default::default()};

        cpu.set_carry(false);
        assert_eq!(cpu.get_carry(), false);
        cpu.set_carry(true);
        assert_eq!(cpu.get_carry(), true);
        cpu.set_carry(false);
        assert_eq!(cpu.get_carry(), false);
    }

    #[test]
    fn test_flag_zero() {
        let mut cpu = CPU {..Default::default()};

        cpu.set_zero(false);
        assert_eq!(cpu.get_zero(), false);
        cpu.set_zero(true);
        assert_eq!(cpu.get_zero(), true);
        cpu.set_zero(false);
        assert_eq!(cpu.get_zero(), false);
    }

    #[test]
    fn test_flag_interrupt_disable() {
        let mut cpu = CPU {..Default::default()};
        
        cpu.set_interrupt_disable(false);
        assert_eq!(cpu.get_interrupt_disable(), false);
        cpu.set_interrupt_disable(true);
        assert_eq!(cpu.get_interrupt_disable(), true);
        cpu.set_interrupt_disable(false);
        assert_eq!(cpu.get_interrupt_disable(), false);
    }

    #[test]
    fn test_flag_decimal_mode() {
        let mut cpu = CPU {..Default::default()};

        cpu.set_decimal_mode(false);
        assert_eq!(cpu.get_decimal_mode(), false);
        cpu.set_decimal_mode(true);
        assert_eq!(cpu.get_decimal_mode(), true);
        cpu.set_decimal_mode(false);
        assert_eq!(cpu.get_decimal_mode(), false);
    }

    #[test]
    fn test_flag_break_command() {
        let mut cpu = CPU {..Default::default()};

        cpu.set_break_command(false);
        assert_eq!(cpu.get_break_command(), false);
        cpu.set_break_command(true);
        assert_eq!(cpu.get_break_command(), true);
        cpu.set_break_command(false);
        assert_eq!(cpu.get_break_command(), false);
    }

    #[test]
    fn test_flag_overflow() {
        let mut cpu = CPU {..Default::default()};

        cpu.set_overflow(false);
        assert_eq!(cpu.get_overflow(), false);
        cpu.set_overflow(true);
        assert_eq!(cpu.get_overflow(), true);
        cpu.set_overflow(false);
        assert_eq!(cpu.get_overflow(), false);
    }

    #[test]
    fn test_flag_negative() {
        let mut cpu = CPU {..Default::default()};

        cpu.set_negative(false);
        assert_eq!(cpu.get_negative(), false);
        cpu.set_negative(true);
        assert_eq!(cpu.get_negative(), true);
        cpu.set_negative(false);
        assert_eq!(cpu.get_negative(), false);
    }

    #[test]
    fn test_fetch_instruction() {
        let mut cpu = CPU {..Default::default()};
        let cpu_copy = cpu.clone();
        let mut memory = Memory {ram: [0u8; 0xffff]};
        memory.ram[0x0000] = Instruction::LDA_IM.into();

        let instruction = cpu.fetch_instruction(&mut memory);

        assert_eq!(cpu.pc, cpu_copy.pc + 1);
        assert_eq!(<Instruction as Into<u8>>::into(instruction), Instruction::LDA_IM.into());
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU {..Default::default()};
        let cpu_copy = cpu.clone();
        let mut mem = Memory { ram: [0u8; 0xffff]};
        cpu.reset();

        let values = [0u8, 69, (!10u8 + 1)];

        for i in 0..3 {
            mem.ram[i*2    ] = Instruction::LDA_IM.into();
            mem.ram[i*2 + 1] = values[i];
        }

        for value in values {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&mem);
            cpu.execute(&mut mem, instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.pc, pc + 2);
            assert_eq!(cpu.cycles, cycles + 2);
            assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
            assert_eq!(cpu.get_zero(), value == 0);
            assert_eq!(cpu.get_interrupt_disable(), cpu_copy.get_interrupt_disable());
            assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
            assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
            assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
            assert_eq!(cpu.get_negative(), (value as i8) < 0);
        }
    }

    #[test]
    fn test_lda_zero_page() {
        let mut cpu = CPU {..Default::default()};
        let cpu_copy = cpu.clone();
        let mut mem = Memory {..Default::default()};

        cpu.reset();

        let values = [0u8, 13, (!105u8 + 1)];
        let addresses = [0x13, 0x5A, 0xff];

        for i in 0..3 {
            mem.ram[2*i + 0] = Instruction::LDA_ZP.into();
            mem.ram[2*i + 1] = addresses[i];
            mem.ram[addresses[i] as usize] = values[i];
        }

        for value in values {
            let pc = cpu.pc;
            let cycles = cpu.cycles;
            let instruction = cpu.fetch_instruction(&mem);
            cpu.execute(&mut mem, instruction);

            assert_eq!(cpu.a, value);
            assert_eq!(cpu.pc, pc + 3);
            assert_eq!(cpu.cycles, cycles + 3);
            assert_eq!(cpu.get_carry(), cpu_copy.get_carry());
            assert_eq!(cpu.get_zero(), value == 0);
            assert_eq!(cpu.get_interrupt_disable(), cpu_copy.get_interrupt_disable());
            assert_eq!(cpu.get_decimal_mode(), cpu_copy.get_decimal_mode());
            assert_eq!(cpu.get_break_command(), cpu_copy.get_break_command());
            assert_eq!(cpu.get_overflow(), cpu_copy.get_overflow());
            assert_eq!(cpu.get_negative(), (value as i8) < 0);
        }
    }

    #[test]
    fn test_lda_zero_page_x() {
        todo!();
    }

    #[test]
    fn test_lda_absolute() {
        todo!()
    }

    #[test]
    fn test_lda_absolute_x() {
        todo!()
    }

    #[test]
    fn test_lda_absolute_y() {
        todo!()
    }

    #[test]
    fn test_lda_indirect_x() {
        todo!()
    }

    #[test]
    fn test_lda_indirect_y() {
        todo!()
    }
}

fn main() {
    let mut cpu = CPU {..Default::default()};
    let mut memory = Memory { ram: [0u8; 0xffff] };

    cpu.reset();

    cpu.execute(&mut memory, Instruction::LDA_IM);
}
