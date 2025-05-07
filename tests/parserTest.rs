pub struct VM {
    pub pc: usize,        // program counter
    pub sp: usize,        // stack pointer
    pub bp: usize,        // base pointer
    pub ax: i64,          // accumulator
    pub stack: Vec<i64>,  // stack memory
    pub text: Vec<i64>,   // bytecode
    pub data: Vec<i64>,   // global/static variables
    pub running: bool,    // execution flag
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
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            let op = self.text[self.pc];
            self.pc += 1;

            match op {
                0 => { // LEA: Load effective address
                    let addr = self.bp + self.text[self.pc] as usize;
                    self.pc += 1;
                    self.ax = addr as i64;
                }

                1 => { // IMM: Load immediate value
                    self.ax = self.text[self.pc];
                    self.pc += 1;
                }

                2 => { // JMP: Unconditional jump
                    self.pc = self.text[self.pc] as usize;
                }

                3 => { // JSR: Jump to subroutine
                    self.sp -= 1;
                    self.stack[self.sp] = self.pc as i64;
                    self.pc = self.text[self.pc] as usize;
                }

                4 => { // BZ: Branch if zero
                    let target = self.text[self.pc] as usize;
                    self.pc += 1;
                    if self.ax == 0 {
                        self.pc = target;
                    }
                }

                5 => { // BNZ: Branch if not zero
                    let target = self.text[self.pc] as usize;
                    self.pc += 1;
                    if self.ax != 0 {
                        self.pc = target;
                    }
                }

                6 => { // ENT: Enter subroutine (setup stack frame)
                    self.sp -= 1;
                    self.stack[self.sp] = self.bp as i64;
                    self.bp = self.sp;
                    self.sp -= self.text[self.pc] as usize;
                    self.pc += 1;
                }

                7 => { // ADJ: Adjust stack
                    self.sp += self.text[self.pc] as usize;
                    self.pc += 1;
                }

                8 => { // LEV: Leave subroutine (restore frame)
                    self.sp = self.bp;
                    self.bp = self.stack[self.sp] as usize;
                    self.sp += 1;
                    self.pc = self.stack[self.sp] as usize;
                    self.sp += 1;
                }

                9 => { // LI: Load int
                    self.ax = self.stack[self.ax as usize];
                }

                10 => { // LC: Load char
                    self.ax = self.stack[self.ax as usize] & 0xFF;
                }

                11 => { // SI: Store int
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    self.stack[addr] = self.ax;
                }

                12 => { // SC: Store char
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    self.stack[addr] = self.ax & 0xFF;
                }

                13 => { // PSH
                    self.sp -= 1;
                    self.stack[self.sp] = self.ax;
                }

                14 => self.ax |= self.stack[self.sp],   // OR
                15 => self.ax ^= self.stack[self.sp],   // XOR
                16 => self.ax &= self.stack[self.sp],   // AND
                17 => self.ax = (self.stack[self.sp] == self.ax) as i64, // EQ
                18 => self.ax = (self.stack[self.sp] != self.ax) as i64, // NE
                19 => self.ax = (self.stack[self.sp] < self.ax) as i64,  // LT
                20 => self.ax = (self.stack[self.sp] > self.ax) as i64,  // GT
                21 => self.ax = (self.stack[self.sp] <= self.ax) as i64, // LE
                22 => self.ax = (self.stack[self.sp] >= self.ax) as i64, // GE
                23 => self.ax = self.stack[self.sp] << self.ax, // SHL
                24 => self.ax = self.stack[self.sp] >> self.ax, // SHR
                25 => self.ax = self.stack[self.sp] + self.ax,  // ADD
                26 => self.ax = self.stack[self.sp] - self.ax,  // SUB
                27 => self.ax = self.stack[self.sp] * self.ax,  // MUL
                28 => self.ax = self.stack[self.sp] / self.ax,  // DIV
                29 => self.ax = self.stack[self.sp] % self.ax,  // MOD

                38 => { // EXIT
                    println!("Program exited with value: {}", self.ax);
                    self.running = false;
                }

                _ => {
                    panic!("Unknown instruction: {}", op);
                }
            }

            // Most binary operations pop from the stack:
            if matches!(op, 14..=29) {
                self.sp += 1;
            }
        }
    }
}
