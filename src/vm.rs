pub struct VM {
    pc: usize,
    sp: usize,
    bp: usize,
    ax: i64,
    stack: Vec<i64>,
    text: Vec<i64>,
    data: Vec<i64>,
}

impl VM {
    pub fn new(text: Vec<i64>, data_size: usize, stack_size: usize) -> Self {
        Self {
            pc: 0,
            sp: stack_size,
            bp: stack_size,
            ax: 0,
            stack: vec![0; stack_size],
            text,
            data: vec![0; data_size],
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = self.text[self.pc];
            self.pc += 1;
            match op {
                1 => { // IMM
                    self.ax = self.text[self.pc];
                    self.pc += 1;
                }
                13 => { // PSH
                    self.sp -= 1;
                    self.stack[self.sp] = self.ax;
                }
                25 => { // ADD
                    self.ax += self.stack[self.sp];
                    self.sp += 1;
                }
                38 => return, // EXIT
                _ => panic!("Unknown instruction: {}", op),
            }
        }
    }
}
