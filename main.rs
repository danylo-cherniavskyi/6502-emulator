use std::default;

type Byte = u8;
type Word = u16;

pub struct CPU {
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
        self.pc = 0;
        self.sp = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = 0;
    }

    pub fn get_carry(&self) -> bool {
        return (self.status & 0x1) != 0;
    }

    pub fn set_carry(&self, value: bool) {
        todo!()
    }

    pub fn get_zero(&self) -> bool {
        return (self.status & 0x2) != 0;
    }

    pub fn set_zero(&self, value: bool) {
        todo!();
    }

    pub fn get_interrupt_disable(&self) -> bool {
        todo!();
    }

    pub fn set_interrupt_disable(&self, value: bool) {
        todo!();
    }

    pub fn get_decimal_mode(&self) -> bool {
        todo!();
    }

    pub fn set_decimal_mode(&self, value: bool) {
        todo!();
    }

    pub fn get_break_command(&self) -> bool {
        todo!();
    }

    pub fn set_break_command(&self, value: bool) {
        todo!();
    }

    pub fn get_overflow(&self) -> bool {
        todo!();
    }

    pub fn set_overflow(&self, value: bool) {
        todo!();
    }

    pub fn get_negative(&self) -> bool {
        todo!();
    }

    pub fn set_negative(&self, value: bool) {
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
        let cpu = CPU {..Default::default()};

        cpu.set_carry(false);
        assert_eq!(cpu.get_carry(), false);
        cpu.set_carry(true);
        assert_eq!(cpu.get_carry(), true);
    }

    #[test]
    fn test_flag_zero() {
        let cpu = CPU {..Default::default()};

        cpu.set_zero(false);
        assert_eq!(cpu.get_zero(), false);
        cpu.set_zero(true);
        assert_eq!(cpu.get_zero(), true);
    }

    #[test]
    fn test_flag_interrupt_disable() {
        let cpu = CPU {..Default::default()};
        
        cpu.set_interrupt_disable(false);
        assert_eq!(cpu.get_interrupt_disable(), false);
        cpu.set_interrupt_disable(true);
        assert_eq!(cpu.get_interrupt_disable(), true);
    }

    #[test]
    fn test_flag_decimal_mode() {
        let cpu = CPU {..Default::default()};

        cpu.set_decimal_mode(false);
        assert_eq!(cpu.get_decimal_mode(), false);
        cpu.set_decimal_mode(true);
        assert_eq!(cpu.get_decimal_mode(), true);
    }

    #[test]
    fn test_flag_break_command() {
        let cpu = CPU {..Default::default()};

        cpu.set_break_command(false);
        assert_eq!(cpu.get_break_command(), false);
        cpu.set_break_command(true);
        assert_eq!(cpu.get_break_command(), true);
    }

    #[test]
    fn test_flag_overflow() {
        let cpu = CPU {..Default::default()};

        cpu.set_overflow(false);
        assert_eq!(cpu.get_overflow(), false);
        cpu.set_overflow(true);
        assert_eq!(cpu.get_overflow(), true);
    }

    #[test]
    fn test_flag_negative() {
        let cpu = CPU {..Default::default()};

        cpu.set_negative(false);
        assert_eq!(cpu.get_negative(), false);
        cpu.set_negative(true);
        assert_eq!(cpu.get_negative(), true);
    }
}

fn main() {
    let mut cpu = CPU {..Default::default()};

    cpu.reset();
}
