pub struct VM {
    pub pc: usize,        // program counter - points to the current instruction in the text
    pub sp: usize,        // stack pointer - points to the top of the stack
    pub bp: usize,        // base pointer - used to manage stack frames for function calls
    pub ax: i64,          // accumulator - used to hold intermediate values/results
    pub stack: Vec<i64>,  // stack memory - used for storing variables and return addresses
    pub text: Vec<i64>,   // bytecode - holds instructions and their operands
    pub data: Vec<i64>,   // global/static variables memory
    pub running: bool,    // execution flag - indicates whether VM should continue running
}

impl VM {
    // Constructor for VM: Initializes registers and allocates memory for stack and data
    pub fn new(text: Vec<i64>, data_size: usize, stack_size: usize) -> Self {
        Self {
            pc: 0,                         // start execution at beginning of text
            sp: stack_size,               // stack pointer starts at top of allocated stack
            bp: stack_size,               // base pointer also starts at top
            ax: 0,                        // accumulator starts with 0
            stack: vec![0; stack_size],   // preallocated stack space
            text,                         // program instructions
            data: vec![0; data_size],     // preallocated data section
            running: true,                // set VM as running
        }
    }

    // Main execution loop for the VM
    pub fn run(&mut self) {
        while self.running {
            let op = self.text[self.pc];  // fetch instruction
            self.pc += 1;                 // advance to next bytecode

            match op {
                0 => { // LEA: Load effective address
                    let addr = self.bp + self.text[self.pc] as usize; // calculate address relative to base pointer
                    self.pc += 1;
                    self.ax = addr as i64; // store in accumulator
                }

                1 => { // IMM: Load immediate value
                    self.ax = self.text[self.pc]; // load immediate value into accumulator
                    self.pc += 1;
                }

                2 => { // JMP: Unconditional jump
                    self.pc = self.text[self.pc] as usize; // set program counter to new location
                }

                3 => { // JSR: Jump to subroutine
                    self.sp -= 1;                         // make space on stack
                    self.stack[self.sp] = self.pc as i64; // save return address
                    self.pc = self.text[self.pc] as usize; // jump to subroutine
                }

                4 => { // BZ: Branch if zero
                    let target = self.text[self.pc] as usize;
                    self.pc += 1;
                    if self.ax == 0 {
                        self.pc = target; // jump if accumulator is zero
                    }
                }

                5 => { // BNZ: Branch if not zero
                    let target = self.text[self.pc] as usize;
                    self.pc += 1;
                    if self.ax != 0 {
                        self.pc = target; // jump if accumulator is not zero
                    }
                }

                6 => { // ENT: Enter subroutine (setup stack frame)
                    self.sp -= 1;
                    self.stack[self.sp] = self.bp as i64;          // save current base pointer
                    self.bp = self.sp;                             // set new base pointer
                    self.sp -= self.text[self.pc] as usize;        // allocate space for local variables
                    self.pc += 1;
                }

                7 => { // ADJ: Adjust stack
                    self.sp += self.text[self.pc] as usize; // deallocate local variables
                    self.pc += 1;
                }

                8 => { // LEV: Leave subroutine (restore frame)
                    self.sp = self.bp;                             // reset stack pointer to base
                    self.bp = self.stack[self.sp] as usize;        // restore old base pointer
                    self.sp += 1;
                    self.pc = self.stack[self.sp] as usize;        // return to saved return address
                    self.sp += 1;
                }

                9 => { // LI: Load integer from stack address in ax
                    self.ax = self.stack[self.ax as usize];
                }

                10 => { // LC: Load character (8 bits)
                    self.ax = self.stack[self.ax as usize] & 0xFF;
                }

                11 => { // SI: Store integer to address on stack
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    self.stack[addr] = self.ax;
                }

                12 => { // SC: Store character (lower 8 bits of ax)
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    self.stack[addr] = self.ax & 0xFF;
                }

                13 => { // PSH: Push accumulator onto stack
                    self.sp -= 1;
                    self.stack[self.sp] = self.ax;
                }

                // Binary operations: perform op with top of stack and ax
                14 => self.ax |= self.stack[self.sp],   // OR
                15 => self.ax ^= self.stack[self.sp],   // XOR
                16 => self.ax &= self.stack[self.sp],   // AND
                17 => self.ax = (self.stack[self.sp] == self.ax) as i64, // EQ (equal)
                18 => self.ax = (self.stack[self.sp] != self.ax) as i64, // NE (not equal)
                19 => self.ax = (self.stack[self.sp] < self.ax) as i64,  // LT (less than)
                20 => self.ax = (self.stack[self.sp] > self.ax) as i64,  // GT (greater than)
                21 => self.ax = (self.stack[self.sp] <= self.ax) as i64, // LE (less or equal)
                22 => self.ax = (self.stack[self.sp] >= self.ax) as i64, // GE (greater or equal)
                23 => self.ax = self.stack[self.sp] << self.ax, // SHL (shift left)
                24 => self.ax = self.stack[self.sp] >> self.ax, // SHR (shift right)
                25 => self.ax = self.stack[self.sp] + self.ax,  // ADD
                26 => self.ax = self.stack[self.sp] - self.ax,  // SUB
                27 => self.ax = self.stack[self.sp] * self.ax,  // MUL
                28 => self.ax = self.stack[self.sp] / self.ax,  // DIV
                29 => self.ax = self.stack[self.sp] % self.ax,  // MOD

                38 => { // EXIT: End program
                    println!("Program exited with value: {}", self.ax); // print exit value
                    self.running = false; // stop execution
                }

                _ => {
                    panic!("Unknown instruction: {}", op); // error for unknown opcodes
                }
            }

            // Most binary operations consume one stack value
            if matches!(op, 14..=29) {
                self.sp += 1;
            }
        }
    }
}
