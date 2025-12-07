pub const STACK_SIZE: usize = 256;

pub struct Stack {
    data: [i32; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
        if self.is_full() {
            panic!("Push onto full stack attempted")
        }

        self.data[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Pop of empty stack attempted")
        }

        self.sp -= 1;
        self.data[self.sp]
    }

    pub fn peek(&self) -> i32 {
        if self.is_empty() {
            panic!("Peek of empty stack attempted")
        }

        self.data[self.sp - 1]
    }

    pub fn is_empty(&self) -> bool {
        self.sp == 0
    }

    pub fn is_full(&self) -> bool {
        self.sp == STACK_SIZE
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
    fn push_out_of_bounds() {
        let mut s = Stack::new();
        (0..STACK_SIZE + 1).for_each(|_| s.push(1));
    }
}
