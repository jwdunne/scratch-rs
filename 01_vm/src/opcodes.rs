#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Push(u16),
    Pop,
    Add,
    Sub,
    Mul,
    Jz(u16),
    Print,
    Halt,
}

impl Opcode {
    pub fn decode(word: u16) -> Self {
        let opcode = (word >> 12) as u8;
        let arg = word & 0xFFF;

        match opcode {
            0x0 => Opcode::Push(arg),
            0x1 => Opcode::Pop,
            0x2 => Opcode::Add,
            0x3 => Opcode::Sub,
            0x4 => Opcode::Mul,
            0x5 => Opcode::Jz(arg),
            0x6 => Opcode::Print,
            0x7 => Opcode::Halt,
            _ => panic!("Unrecognised opcode"),
        }
    }

    pub fn encode(&self) -> u16 {
        match self {
            Opcode::Push(arg) => arg & 0xFFF,
            Opcode::Pop => 0x1000,
            Opcode::Add => 0x2000,
            Opcode::Sub => 0x3000,
            Opcode::Mul => 0x4000,
            Opcode::Jz(arg) => 0x5000 | (arg & 0xFFF),
            Opcode::Print => 0x6000,
            Opcode::Halt => 0x7000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unrecognised_opcode_panics() {
        Opcode::decode(0x9000);
    }

    #[test]
    fn decode_no_arg() {
        assert_eq!(Opcode::decode(0x0004), Opcode::Push(0x0004))
    }

    #[test]
    fn decode_with_arg() {
        assert_eq!(Opcode::decode(0x1000), Opcode::Pop);
    }

    #[test]
    fn encode_and_decode() {
        assert_eq!(Opcode::decode(Opcode::Push(5).encode()), Opcode::Push(5))
    }
}
