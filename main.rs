type Byte = u8;
type Word = u16;

struct CPU {
    pc: Word,
    sp: Byte,
    a: Byte,
    x: Byte,
    y: Byte,
    status: Byte,
}

impl CPU {
    fn reset(mut self) {
        self.pc = 0;
        self.sp = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = 0;
    }
}

fn main() {
    let cpu = CPU {
        pc: 0,
        sp: 0,
        a: 0,
        x: 0,
        y: 0,
        status: 0,
    };

    cpu.reset();
}