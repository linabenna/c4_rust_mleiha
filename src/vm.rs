// Constants for tokens and opcodes- add more

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// #[repr(i32)]
// enum Token {
//     None, 
//     Num(i32),
//     Id(String),
//     Char(char),
//     Str(String),
//     Else, Enum, If, Int, Return, Sizeof, While, Assign, Cond, Lor, Lan, Or,
//     Xor, And, Eq, Ne, Lt, Le, Gt, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec,
//     Brak, 
//     // RBrak,
//     LParen = b'(' as i32,
//     RParen = b')' as i32, 
//     // LBrace, 
//     // RBrace, 
//     Comma = b',' as i32, 
//     Colon = b':' as i32, 
//     Semicolon = b';' as i32, 
//     Not = b'!' as i32,
//     BitNot = b'~' as i32,
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    None, 
    Num(i32),
    Id(String),
    Char(char),
    Str(String),
    Else, Enum, If, Int, Return, Sizeof, While, 
    Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Lt, Shl, Add, Mul, Inc,
    Ne, Le, Gt, Ge, Shr, Sub, Div, Mod, Dec,
    Brak,
    LParen, RParen, Comma, Colon, Semicolon, Not, BitNot,
}

impl Token {
    fn precedence(&self) -> Option<i32> {
        use Token::*;
        Some(match self {
            Assign => 1,
            Cond   => 2,
            Lor    => 3,
            Lan    => 4,
            Or     => 5,
            Xor    => 6,
            And    => 7,
            Eq     => 8,
            Lt     => 9,
            Shl    => 10,
            Add    => 11,
            Mul    => 12,
            Inc    => 13,
            _ => return Some(0),
        })
    }
}


const Num: i32 = 128;
const Id: i32 = 129;

// these are opcode constants the vm can execute
const LEA: i32 = 0; // load effective address
const IMM: i32 = 1; // load immediate value
const JMP: i32 = 2; // unconditional jump
const JSR: i32 = 3; // jump to subroutine (function call)
const BZ: i32 = 4; // branch if zero
const BNZ: i32 = 5; // branch if not zero
const ENT: i32 = 6; // enter function (setup stack frame)
const ADJ: i32 = 7; // adjust stack
const LEV: i32 = 8; // leave function
const LI: i32 = 9; // load integer from memory
const LC: i32 = 10; // load character from memory
const SI: i32 = 11; // store integer to memory
const SC: i32 = 12; // store character to memory
const PSH: i32 = 13; // push value onto stack

// the rest below are arithmetic and logical operations
const OR: i32 = 14;
const XOR: i32 = 15;
const AND: i32 = 16;
const EQ: i32 = 17;
const NE: i32 = 18;
const LT: i32 = 19;
const GT: i32 = 20;
const LE: i32 = 21;
const GE: i32 = 22;
const SHL: i32 = 23;
const SHR: i32 = 24;
const ADD: i32 = 25;
const SUB: i32 = 26;
const MUL: i32 = 27;
const DIV: i32 = 28;
const MOD: i32 = 29;

// below are system calls
const OPEN: i32 = 30;
const READ: i32 = 31;
const CLOS: i32 = 32;
const PRTF: i32 = 33;
const MALC: i32 = 34;
const FREE: i32 = 35;
const MSET: i32 = 36;
const MCMP: i32 = 37;
const EXIT: i32 = 38;

// data types
const CHAR: i32 = 0;
const INT: i32 = 1;
const PTR: i32 = 2;

#[derive(Debug)]
struct Parser {
    e: Vec<i32>, // Emitted code
    tk: Token,     // Current token
    ival: i32,   // Current token value
    ty: i32,     // Current expression type
    loc: i32,    // Local variable offset
    line: i32,   // Current line number
    data: Vec<u8>, // <--- memory area to simulate global string storage
    pos: i32, 
}

impl Parser {
    fn new() -> Self {
        Self {
            e: Vec::new(),
            tk: Token::None, // defining anything for now
            ival: 0,
            ty: 0,
            loc: 0,
            line: 1,
            data: Vec::new(),
            pos: 0,
        }
    }

    fn next(&mut self) {
        // here, given the tokens generated from the parser, 
        // implement a function that advances through each token
        // Logic to get the next token
        // if self.pos < self.tokens.len() {
        //     self.tk = self.tokens[self.pos].clone();
        //     self.pos += 1;
        // } else {
        //     self.tk = Token::None;
        // }
    }

    fn expr(&mut self, lev: i32) {
        let token = self.tk.clone();

        match token {
            Token::None => {
                eprintln!("{}: unexpected eof in expression", self.line);
                std::process::exit(-1);
            }
            Token::Num(val) => {
                self.e.push(IMM);
                self.e.push(val);
                self.next();
                self.ty = INT;
            }
            Token::Str(s) => {
                self.e.push(IMM);
                let addr = self.store_string(&s); // Now no conflict with borrowing `self`
                self.e.push(addr);
                self.next();
                self.ty = INT; // Or PTR, depending on your needs
            }

            Token::Sizeof => {
                self.next();
                if self.tk != Token::LParen {
                    eprintln!("{}: open paren expected in sizeof", self.line);
                    std::process::exit(-1);
                }
                self.next();
            
                self.ty = INT;
                if self.tk == Token::Int {
                    self.next();
                } else if matches!(self.tk, Token::Char(_)) {
                    self.next();
                    self.ty = CHAR;
                } else {
                    eprintln!("{}: type name expected in sizeof", self.line);
                    std::process::exit(-1);
                }
                while self.tk == Token::Mul {
                    self.next();
                    self.ty += PTR;
                }
                if self.tk != Token::RParen {
                    eprintln!("{}: close paren expected in sizeof", self.line);
                    std::process::exit(-1);
                }
                self.next();
            
                self.e.push(IMM);
                self.e.push(if self.ty == CHAR { 1 } else { 4 }); // assuming sizeof(char) = 1, sizeof(int) = 4
                self.ty = INT;
            }         

            Token::Id(name) => {
                // Logic for identifier (e.g. variable or function)
                self.next();
            }

            Token::Mul => {
                self.next();
                self.expr(Token::Inc.precedence().unwrap());
                if (self.ty == INT){
                    self.ty = self.ty - PTR;
                } else {
                    eprintln!("{}: close paren expected in sizeof", self.line);
                    std::process::exit(-1);
                }
                self.e.push(if self.ty == CHAR { LC } else { LI })
            }

            Token::And => {
                self.next();
                self.expr(Token::Inc.precedence().unwrap());
                
                if let Some(&last) = self.e.last() {
                    if last == LC || last == LI {
                        self.e.pop();
                    } else {
                        eprintln!("{}: bad address-of", self.line);
                        std::process::exit(-1);
                    }
                } else {
                    eprintln!("{}: close paren expected in sizeof", self.line);
                    std::process::exit(-1);
                }
                self.ty = self.ty + PTR;
            }

            Token::Not => {
                self.next();
                self.expr(Token::Inc.precedence().unwrap()); // Inc is the precedence level
                self.e.push(PSH);
                self.e.push(IMM);
                self.e.push(0);
                self.e.push(EQ);
                self.ty = INT;
            }
            Token::BitNot => {
                self.next();
                self.expr(Token::Inc.precedence().unwrap());
                self.e.push(PSH);
                self.e.push(IMM);
                self.e.push(-1);
                self.e.push(XOR);
                self.ty = INT;
            }
            Token::Add => {
                self.next();
                self.expr(Token::Inc.precedence().unwrap());
                self.ty = INT;
            }
            
            Token::Sub => {
                self.next();
                self.e.push(IMM);
                match self.tk.clone() {
                    Token::Num(val) => {
                        self.e.push(-val);
                        self.next();
                    }
                    _ => {
                        self.e.push(-1);
                        self.e.push(PSH);
                        self.expr(Token::Inc.precedence().unwrap());
                        self.e.push(MUL);
                    }
                }
                self.ty = INT;
            }


            _ => {
                eprintln!("{}: bad expression", self.line);
                std::process::exit(-1);
            }
        }
    
        // precedence climbing would go here
    }

    fn store_string(&mut self, s: &str) -> i32 {
        // Align to 4 bytes (simulate C4's `sizeof(int) & -sizeof(int)`)
        while self.data.len() % 4 != 0 {
            self.data.push(0);
        }

        let address = self.data.len() as i32; // get the current offset (address)

        // Store the string bytes
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0); // null-terminator

        // Optional: align after string for next storage
        while self.data.len() % 4 != 0 {
            self.data.push(0);
        }

        address
    }
}


fn main() {
    let mut src = false;
    let mut debug = false;
    let args: Vec<String> = std::env::args().collect();
    
    let mut argc = args.len() - 1; // Exclude the program name
    let mut argv = &args[1..];
  
    if argc > 0 && argv[0] == "-s" {
        src = true;
        argc -= 1;
        argv = &argv[1..];
    }
    if argc > 0 && argv[0] == "-d" {
        debug = true;
        argc -= 1;
        argv = &argv[1..];
    }
    if argc < 1 {
        eprintln!("usage: c4 [-s] [-d] file ...");
        return;
    }
  
    let file_path = &argv[0];
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("could not open({})", file_path);
            return;
        }
    };
  
    // Allocate memory pools
    let sym = vec![0; POOL_SIZE]; // Symbol area
    let e = vec![0; POOL_SIZE]; // Text area
    let data = vec![0; POOL_SIZE]; // Data area
    let sp = vec![0; POOL_SIZE]; // Stack area
  
    // Read the source file
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
  
    // Initialize the lexer
    let mut lexer = Lexer::new(&source);
    
    // Parse declarations
    let mut line = 1;
    let mut tk = lexer.next_token(); // Get the first token
  
    while tk != Token::None {
        let mut bt = INT; // Base type
        if tk == Token::Int {
            tk = lexer.next_token();
        } else if tk == Token::Char {
            tk = lexer.next_token();
            bt = CHAR;
        } else if tk == Token::Enum {
            tk = lexer.next_token();
            if tk == Token::LBrace {
                tk = lexer.next_token();
                let mut i = 0;
                while tk != Token::RBrace {
                    if tk != Token::Id {
                        eprintln!("{}: bad enum identifier {}", line, tk);
                        return;
                    }
                    tk = lexer.next_token();
                    if tk == Token::Assign {
                        tk = lexer.next_token();
                        if tk != Token::Num {
                            eprintln!("{}: bad enum initializer", line);
                            return;
                        }
                        i = ival; // Assuming ival is defined somewhere
                        tk = lexer.next_token();
                    }
                    // Add to symbol table
                    // Assuming you have a way to add to the symbol table
                    // id[Class] = Num; id[Type] = INT; id[Val] = i++;
                    if tk == Token::Comma {
                        tk = lexer.next_token();
                    }
                }
                tk = lexer.next_token();
            }
        }
  
        // Handle global declarations
        while tk != Token::Semicolon && tk != Token::RBrace {
            let mut ty = bt;
            while tk == Token::Mul {
                tk = lexer.next_token();
                ty += PTR; // Assuming PTR is defined
            }
            if tk != Token::Id {
                eprintln!("{}: bad global declaration", line);
                return;
            }
            // Check for duplicate definitions
            // Assuming you have a way to check for duplicates
            tk = lexer.next_token();
            // id[Type] = ty; // Assuming you have a way to set the type
  
            if tk == Token::LParen { // Function
                // Handle function declaration
                // Similar to the C code, handle parameters and local declarations
            } else {
                // Handle global variable declaration
                // id[Class] = Glo; id[Val] = (int)data; // Assuming you have a way to set these
                // data += std::mem::size_of::<i32>(); // Assuming you have a way to manage data
            }
            if tk == Token::Comma {
                tk = lexer.next_token();
            }
        }
        tk = lexer.next_token();
    }
  
    // Assuming idmain is defined and points to the main function
    let pc = idmain.val as *mut i32; // Get the program counter from the main function
    if src {
        return;
    }
  
    // Setup stack
    let mut bp = sp.as_mut_ptr().add(POOL_SIZE);
    let mut sp = bp;
    unsafe {
        *sp = EXIT; // Call exit if main returns
        sp = sp.sub(1);
        *sp = PSH; // Push the return address
        sp = sp.sub(1);
        *sp = argc as i32; // Push argc
        sp = sp.sub(1);
        *sp = argv.as_ptr() as i32; // Push argv
        sp = sp.sub(1);
        *sp = t as i32; // Push the temporary pointer
    }
  
    // Run the virtual machine
    let mut cycle = 0;
    loop {
        let i = unsafe { *pc }; // Fetch the instruction
        pc = pc.add(1); // Move to the next instruction
        cycle += 1;
  
        if debug {
            println!("{}> {:?}", cycle, i); // Print debug information
        }
  
        match i {
            LEA => {
                let a = unsafe { *(bp.add(*pc)) }; // Load local address
                pc = pc.add(1);
            }
            IMM => {
                let a = unsafe { *pc }; // Load immediate value
                pc = pc.add(1);
            }
            JMP => {
                pc = unsafe { *(pc as *mut *mut i32) }; // Jump
            }
            JSR => {
                sp = sp.sub(1);
                unsafe { *sp = (pc as i32) + 1 }; // Save return address
                pc = unsafe { *(pc as *mut *mut i32) }; // Jump to subroutine
            }
            BZ => {
                if a == 0 {
                    pc = unsafe { *(pc as *mut *mut i32) }; // Branch if zero
                } else {
                    pc = pc.add(1);
                }
            }
            BNZ => {
                if a != 0 {
                    pc = unsafe { *(pc as *mut *mut i32) }; // Branch if not zero
                } else {
                    pc = pc.add(1);
                }
            }
            ENT => {
                sp = sp.sub(1);
                unsafe { *sp = bp as i32 }; // Enter subroutine
                bp = sp;
                sp = sp.sub(*pc); // Adjust stack pointer
                pc = pc.add(1);
            }
            ADJ => {
                sp = sp.add(*pc); // Stack adjust
                pc = pc.add(1);
            }
            LEV => {
                sp = bp; // Leave subroutine
                bp = unsafe { *sp as *mut i32 };
                pc = unsafe { *sp.add(1) as *mut i32 };
            }
            LI => {
                a = unsafe { *(a as *mut i32) }; // Load integer
            }
            LC => {
                a = unsafe { *(a as *mut u8) }; // Load character
            }
            SI => {
                unsafe { *(sp as *mut i32) = a }; // Store integer
                sp = sp.add(1);
            }
            SC => {
                unsafe { *(sp as *mut u8) = a as u8 }; // Store character
                sp = sp.add(1);
            }
            PSH => {
                sp = sp.sub(1);
                unsafe { *sp = a }; // Push value
            }
            // Handle other operations (OR, XOR, AND, etc.) similarly...
            EXIT => {
                println!("exit({}) cycle = {}", *sp, cycle);
                return; // Exit the program
            }
            _ => {
                eprintln!("unknown instruction = {}! cycle = {}", i, cycle);
                return; // Handle unknown instruction
            }
        }
    }
  }
  