use std::str::FromStr;

pub const STACK_SIZE: usize = 256;

struct Stack {
    data: [i32; STACK_SIZE],
    sp: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    fn push(&mut self, value: i32) {
        if self.is_full() {
            panic!("Push onto full stack attempted")
        }

        self.data[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Pop of empty stack attempted")
        }

        self.sp -= 1;
        self.data[self.sp]
    }

    #[warn(dead_code)]
    fn peek(&self) -> i32 {
        if self.is_empty() {
            panic!("Peek of empty stack attempted")
        }

        self.data[self.sp - 1]
    }

    fn is_empty(&self) -> bool {
        self.sp == 0
    }

    fn is_full(&self) -> bool {
        self.sp == STACK_SIZE
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
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
    fn decode(word: u16) -> Self {
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

    #[warn(dead_code)]
    fn encode(&self) -> u16 {
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

pub struct Vm {
    stack: Stack,
    code: Vec<u16>,
    pc: usize,
    halted: bool,
}

impl Vm {
    pub fn new(code: Vec<u16>) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
            halted: false,
        }
    }

    pub fn step(&mut self) {
        if self.halted || self.pc >= self.code.len() {
            self.halted = true;
            return;
        }

        let instruction = Opcode::decode(self.code[self.pc]);
        self.pc += 1;

        match instruction {
            Opcode::Push(value) => {
                self.stack.push(value as i32);
            }
            Opcode::Pop => {
                self.stack.pop();
            }
            Opcode::Add => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a + b);
            }
            Opcode::Sub => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b - a);
            }
            Opcode::Mul => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a * b);
            }
            Opcode::Jz(addr) => {
                if self.stack.pop() == 0 {
                    self.pc = addr as usize;
                }
            }
            Opcode::Print => {
                let value = self.stack.pop();
                print!("{}", value);
            }
            Opcode::Halt => {
                self.halted = true;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn pop_empty_stack() {
        let mut s = Stack::new();
        s.pop();
    }

    #[test]
    #[should_panic]
    fn peek_empty_stack() {
        let s = Stack::new();
        s.peek();
    }

    #[test]
    #[should_panic]
    fn push_out_of_stack_bounds() {
        let mut s = Stack::new();
        (0..STACK_SIZE + 1).for_each(|_| s.push(1));
    }

    #[test]
    #[should_panic]
    fn unrecognised_opcode_panics() {
        Opcode::decode(0x9000);
    }

    #[test]
    fn decode_opcode_no_arg() {
        assert_eq!(Opcode::decode(0x0004), Opcode::Push(0x0004))
    }

    #[test]
    fn decode_opcode_with_arg() {
        assert_eq!(Opcode::decode(0x1000), Opcode::Pop);
    }

    #[test]
    fn encode_and_decode_opcode() {
        assert_eq!(Opcode::decode(Opcode::Push(5).encode()), Opcode::Push(5))
    }

    #[test]
    fn vm_performs_arithmetic() {
        let code = vec![
            Opcode::Push(10).encode(),
            Opcode::Push(5).encode(),
            Opcode::Add.encode(),
            Opcode::Push(3).encode(),
            Opcode::Sub.encode(),
        ];

        let mut vm = Vm::new(code);
        vm.run();

        assert_eq!(vm.stack.peek(), 12);
    }

    #[test]
    fn vm_handles_logic() {
        let code = vec![
            Opcode::Push(0).encode(),
            Opcode::Jz(4).encode(),
            Opcode::Push(0).encode(),
            Opcode::Jz(0).encode(),
            Opcode::Push(1).encode(),
        ];

        let mut vm = Vm::new(code);
        vm.run();

        assert_eq!(vm.stack.peek(), 1);
    }
}
