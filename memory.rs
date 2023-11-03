pub mod memory {
    pub type Byte = u8;
    pub type Word = u16;

    pub fn add_mod_256(n1: u8, n2: u8) -> u8 {
        let sum = ((n1 as u16 + n2 as u16) % 256) as u8;
        return sum;
    }

    pub fn add_mod_65536(n1: u16, n2: u16) -> u16 {
        let sum = (n1 as u32 + n2 as u32) % (0xffff + 1);
        return sum as u16;
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum AddressingMode {
        Immediate,
        ZeroPage,
        ZeroPageReg,
        Absolute,
        AbsoluteReg,
        IndirectX,
        IndirectY,
    }

    pub trait MemoryLike<T> {
        fn read(&self, addr: Word) -> T;
        fn write(&mut self, addr: Word, value: T);
        fn read_zero_page(&self, pc: &mut Word) -> T;
        fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> T;
        fn read_absolute(&self, pc: &mut Word) -> T;
        fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> T;
        fn read_absolute_x_check_crossing(
            &self,
            pc: &mut Word,
            x: Byte,
            page_crossed: &mut bool,
        ) -> T;
        fn read_indirect_x(&self, pc: &mut Word, x: Byte) -> T;
        fn read_indirect_y(&self, pc: &mut Word, y: Byte) -> T;
        fn read_indirect_y_check_crossing(
            &self,
            pc: &mut Word,
            y: Byte,
            page_crossed: &mut bool,
        ) -> T;
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Memory {
        pub(crate) ram: [Byte; 0x10000],
    }

    impl MemoryLike<u8> for Memory {
        fn read(&self, addr: Word) -> u8 {
            return self.ram[addr as usize];
        }

        fn write(&mut self, addr: Word, value: u8) {
            self.ram[addr as usize] = value;
        }

        fn read_zero_page(&self, pc: &mut Word) -> u8 {
            let addr_zp: u8 = self.read(*pc);
            let value = self.read(addr_zp as u16);
            *pc += 1;

            return value;
        }

        fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> u8 {
            let addr_zp = self.read(*pc);
            let addr_final = add_mod_256(addr_zp, x);
            let value = self.read(addr_final as u16);
            *pc += 1;

            return value;
        }

        fn read_absolute(&self, pc: &mut Word) -> u8 {
            let addr = self.read(*pc);
            let value = self.read(addr);
            *pc += 2;

            return value;
        }

        fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> u8 {
            let addr = self.read(*pc);
            let addr_final = add_mod_65536(addr, x as u16);
            let value = self.read(addr_final);
            *pc += 2;

            return value;
        }

        fn read_absolute_x_check_crossing(
            &self,
            pc: &mut Word,
            x: Byte,
            page_crossed: &mut bool,
        ) -> u8 {
            let addr = self.read(*pc);
            let addr_final = add_mod_65536(addr, x as u16);
            let value = self.read(addr_final);
            *pc += 2;

            *page_crossed = addr_final > 0xff;

            return value;
        }

        fn read_indirect_x(&self, pc: &mut Word, x: Byte) -> u8 {
            let addr = self.read(*pc);
            let addr_zp = add_mod_256(addr, x);
            let addr_final = self.read(addr_zp as u16);
            let value = self.read(addr_final);
            *pc += 1;

            return value;
        }

        fn read_indirect_y(&self, pc: &mut Word, y: Byte) -> u8 {
            let addr: u8 = self.read(*pc);
            let addr_on_zp = self.read(addr as u16);
            let addr_final = add_mod_65536(addr_on_zp, y as u16);
            let value = self.read(addr_final);
            *pc += 1;

            return value;
        }

        fn read_indirect_y_check_crossing(
            &self,
            pc: &mut Word,
            y: Byte,
            page_crossed: &mut bool,
        ) -> u8 {
            let addr: u8 = self.read(*pc);
            let addr_on_zp = self.read(addr as u16);
            let addr_final = add_mod_65536(addr_on_zp, y as u16);
            let value = self.read(addr_final);
            *pc += 1;

            *page_crossed = addr_final > 0xff;

            return value;
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
            let addr_zp: u8 = self.read(*pc);
            let value = self.read(addr_zp as u16);
            *pc += 1;

            return value;
        }

        fn read_zero_page_x(&self, pc: &mut Word, x: Byte) -> u16 {
            let addr_zp = self.read(*pc);
            let addr_final = add_mod_256(addr_zp, x);
            let value = self.read(addr_final as u16);
            *pc += 1;

            return value;
        }

        fn read_absolute(&self, pc: &mut Word) -> u16 {
            let addr = self.read(*pc);
            let value = self.read(addr);
            *pc += 2;

            return value;
        }

        fn read_absolute_x(&self, pc: &mut Word, x: Byte) -> u16 {
            let addr = self.read(*pc);
            let addr_final = add_mod_65536(addr, x as u16);
            let value = self.read(addr_final);
            *pc += 2;

            return value;
        }

        fn read_absolute_x_check_crossing(
            &self,
            _pc: &mut Word,
            _x: Byte,
            _page_crossed: &mut bool,
        ) -> u16 {
            todo!();
        }

        fn read_indirect_x(&self, _pc: &mut Word, _x: Byte) -> u16 {
            todo!();
        }

        fn read_indirect_y(&self, _pc: &mut Word, _y: Byte) -> u16 {
            todo!();
        }

        fn read_indirect_y_check_crossing(
            &self,
            _pc: &mut Word,
            _y: Byte,
            _page_crossed: &mut bool,
        ) -> u16 {
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

        pub fn write_zero_page(&mut self, pc: &mut Word, value: u8) {
            let addr: u8 = self.read(*pc);
            self.write(addr as u16, value);

            *pc += 1;
        }

        pub fn write_zero_page_x(&mut self, pc: &mut Word, x: u8, value: u8) {
            let addr: u8 = self.read(*pc);
            let addr_final = add_mod_256(addr, x);
            self.write(addr_final as u16, value);

            *pc += 1;
        }

        pub fn write_absolute(&mut self, pc: &mut Word, value: u8) {
            let addr: u16 = self.read(*pc);
            self.write(addr, value);

            *pc += 2;
        }

        pub fn write_absolute_x(&mut self, pc: &mut Word, x: u8, value: u8) {
            let addr: u16 = self.read(*pc);
            let addr_final = add_mod_65536(addr, x as u16);
            self.write(addr_final, value);

            *pc += 2;
        }

        pub fn write_indirect_x(&mut self, pc: &mut Word, x: u8, value: u8) {
            let addr: u8 = self.read(*pc);
            let addr_zp = add_mod_256(addr, x);
            let addr_final: u16 = self.read(addr_zp as u16);
            self.write(addr_final, value);

            *pc += 1;
        }

        pub fn write_indirect_y(&mut self, pc: &mut Word, y: u8, value: u8) {
            let addr: u8 = self.read(*pc);
            let addr_on_zp: u16 = self.read(addr as u16);
            let addr_final = add_mod_65536(addr_on_zp, y as u16);
            self.write(addr_final, value);

            *pc += 1;
        }

        pub fn read_immediate(&self, pc: &mut Word) -> u8 {
            let value = self.read(*pc);
            *pc += 1;

            return value;
        }
    }

    impl Default for Memory {
        fn default() -> Self {
            Memory {
                ram: [0u8; 0x10000],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Memory, MemoryLike};

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
    fn test_write_zero_page() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let pcs_init = [0x0010u16, 0xfffe, 0x1234, 0xabcd];
        let addresses = [0x00u8, 0xff, 0x12, 0xab];
        let values = [0x12u8, 0x00, 0xff, 0x69];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
        }

        for i in 0..4 {
            memory.write_zero_page(&mut pcs[i], values[i]);
            let value: u8 = memory.read(addresses[i] as u16);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_write_zero_page_x() {
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
        }

        for i in 0..4 {
            memory.write_zero_page_x(&mut pcs[i], x_addresses[i], values[i]);
            let value: u8 = memory.read(addresses_final[i] as u16);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_write_absolute() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
        }

        for i in 0..4 {
            memory.write_absolute(&mut pcs[i], values[i]);
            let value: u8 = memory.read(addresses[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_write_absolute_x() {
        let mut memory = Memory {
            ..Default::default()
        };

        let mut pcs = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let pcs_init = [0x0000u16, 0xfffd, 0xABCD, 0x5648];
        let addresses = [0x0010u16, 0xffed, 0x1234, 0xBADC];
        let x_addresses = [0x00u8, 0x05, 0x52, 0x42];
        let addresses_final = [0x0010, 0xfff2, 0x1286, 0xbb1e];
        let values = [0xffu8, 0x00, 0xab, 0x31];

        for i in 0..4 {
            memory.write(pcs[i], addresses[i]);
        }

        for i in 0..4 {
            memory.write_absolute_x(&mut pcs[i], x_addresses[i], values[i]);
            let value: u8 = memory.read(addresses_final[i]);

            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 2);
        }
    }

    #[test]
    fn test_write_indirect_x() {
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
        }

        for i in 0..4 {
            memory.write_indirect_x(&mut pcs[i], x_addresses[i], values[i]);
            let value: u8 = memory.read(addresses_on_zp[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
        }
    }

    #[test]
    fn test_write_indirect_y() {
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
        }

        for i in 0..4 {
            memory.write_indirect_y(&mut pcs[i], y_addresses[i], values[i]);
            let value: u8 = memory.read(addresses_sum[i]);
            assert_eq!(value, values[i]);
            assert_eq!(pcs[i], pcs_init[i] + 1);
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
        let addresses = [0x0010u16, 0xfffeu16, 0x1234u16, 0xABCDu16];
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
        let addresses_final = [0x0010, 0xfff2, 0x1286, 0xbb1e];
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
        let addresses_final = [0x0010, 0xfff2, 0x1286, 0xbb1e];
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
}
