// Constants for tokens and opcodes- add more

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(i32)]
enum Token {
    None, 
    Num(i32),
    Id(String),
    Char(char),
    Str(String),
    Else, Enum, If, Int, Return, Sizeof, While, Assign, Cond, Lor, Lan, Or,
    Xor, And, Eq, Ne, Lt, Le, Gt, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec,
    Brak, 
    // RBrak,
    LParen = b'(' as i32,
    RParen = b')' as i32, 
    // LBrace, 
    // RBrace, 
    Comma = b',' as i32, 
    Colon = b':' as i32, 
    Semicolon = b';' as i32, 
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
        }
    }

    fn next(&mut self) {
        // Logic to get the next token
        // This is a placeholder; actual implementation will depend on your lexer
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
                self.ty = INT; // Or PTR, depending on your needs
            }
            Token::Id(name) => {
                // Logic for identifier (e.g. variable or function)
                self.next();
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
    let mut state = Parser::new();

    // Test with a number
    state.tk = Token::Num(42);
    state.expr(0);
    println!("Emitted code (number): {:?}", state.e);

    // Reset state
    state.e.clear();

    // Test with a string
    state.tk = Token::Str("hello".to_string());
    state.expr(0);
    println!("Emitted code (string): {:?}", state.e);
    println!("Data section (string): {:?}", state.data);
}
