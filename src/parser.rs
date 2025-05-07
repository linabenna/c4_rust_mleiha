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
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    None, 
    Num(i32),
    Id(String),
    Char(char),
    Str(String),
    Else, Enum, If, Int, Return, Sizeof, While, Do,
    Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Lt, Shl, Add, Mul, Inc,
    Ne, Le, Gt, Ge, Shr, Sub, Div, Mod, Dec,
    Brak,
    LParen, RParen, RBrace, LBrace, Comma, Colon, Semicolon, Not, BitNot,
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

#[derive(Debug, Hash, Clone, PartialEq)]
enum Class {
    Sys,  // System function
    Fun,  // User-defined function
    Num,  // Immediate number
    Loc,  // Local variable
    Glo,  // Global variable
}

#[derive(Debug, Hash, Clone)]
struct Symbol {
    class: Class,   // Class of symbol (Sys, Fun, Num, Loc, Glo)
    val: i32,       // Address or value
    typ: i32,       // Type (e.g., INT, CHAR, PTR, etc.)
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
    symbols: HashMap<String, Symbol>,
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
            symbols: HashMap::new(),
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

            Token::Id(ref name) => {
                // Lookup the symbol table entry by identifier name
                // let d = self.symbols.get(name).cloned().unwrap_or_else(|| {
                //     eprintln!("{}: undefined identifier '{}'", self.line, name);
                //     std::process::exit(-1);
                // });
                // self.next();

                if let Some(d) = self.symbols.get(name.as_str()).cloned() {
                    self.next(); // consume identifier

                    if self.tk == Token::LParen {
                        self.next();
                        let mut t = 0;
                        while self.tk != Token::RParen {
                            self.expr(Token::Assign.precedence().unwrap());
                            self.e.push(PSH);
                            t += 1;
                            if self.tk == Token::Comma {
                                self.next();
                            }
                        }
                        self.next();
                        match d.class {
                            Class::Sys => self.e.push(d.val),
                            Class::Fun => {
                                self.e.push(JSR);
                                self.e.push(d.val);
                            }
                            _ => {
                                eprintln!("{}: bad function call", self.line);
                                std::process::exit(-1);
                            }
                        }
                        if t > 0 {
                            self.e.push(ADJ);
                            self.e.push(t);
                        }
                        self.ty = d.typ;
                    } else if d.class == Class::Num {
                        self.e.push(IMM);
                        self.e.push(d.val);
                        self.ty = INT;
                    } else {
                        match d.class {
                            Class::Loc => {
                                self.e.push(LEA);
                                self.e.push(self.loc - d.val);
                            }
                            Class::Glo => {
                                self.e.push(IMM);
                                self.e.push(d.val);
                            }
                            _ => {
                                eprintln!("{}: undefined variable '{}'", self.line, name);
                                std::process::exit(-1);
                            }
                        }
                        self.ty = d.typ;
                        self.e.push(if self.ty == CHAR { LC } else { LI });
                    }
                } else {
                    println!("{}: undefined variable {}", self.line, name);
                    std::process::exit(-1);
                }
                
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

            Token::LParen => {
                self.next(); // consume '('
                match self.tk {
                    Token::Int => {
                        // Handle cast for 'Int'
                        let mut t = INT;  // Initialize the type as INT
                        self.next(); // consume 'Int'
            
                        // Check for pointer dereferencing (*)
                        while let Token::Mul = self.tk {
                            self.next(); // consume '*'
                            t = t + PTR; // Adjust type for pointers
                        }
            
                        // Ensure we have a closing parenthesis ')'
                        if let Token::RParen = self.tk {
                            self.next(); // consume ')'
                        } else {
                            panic!("{}: bad cast", self.line); // Handle bad cast
                        }
            
                        // Handle the casted expression
                        self.expr(Token::Inc.precedence().unwrap());
                        self.ty = t; // Set the type for the expression
                    }
                    Token::Char(_) => {
                        // Handle cast for 'Char'
                        let mut t = CHAR;  // Initialize the type as CHAR
                        self.next(); // consume 'Char'
            
                        // Check for pointer dereferencing (*)
                        while let Token::Mul = self.tk {
                            self.next(); // consume '*'
                            t = t + PTR; // Adjust type for pointers
                        }
            
                        // Ensure we have a closing parenthesis ')'
                        if let Token::RParen = self.tk {
                            self.next(); // consume ')'
                        } else {
                            panic!("{}: bad cast", self.line); // Handle bad cast
                        }
            
                        // Handle the casted expression
                        self.expr(Token::Inc.precedence().unwrap());
                        self.ty = t; // Set the type for the expression
                    }
                    _ => {
                        // Regular parenthesis group
                        self.expr(Token::Assign.precedence().unwrap());
            
                        // Ensure we have a closing parenthesis ')'
                        if let Token::RParen = self.tk {
                            self.next(); // consume ')'
                        } else {
                            panic!("{}: close paren expected", self.line); // Handle missing closing parenthesis
                        }
                    }
                }
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

            Token::Inc | Token::Dec => {
                let t = self.tk.clone(); // Clone the current token (Inc or Dec)
                self.next(); // Consume the token (either Inc or Dec)
                
                // Evaluate the expression for the operand
                self.expr(Token::Inc.precedence().unwrap());
            
                // Handle the left operand type (either LC or LI)
                match self.e.last() {
                    Some(&LC) => {
                        self.e.push(PSH); // Push the operand to the stack
                        self.e.push(LC); // Load the left value
                    }
                    Some(&LI) => {
                        self.e.push(PSH); // Push the operand to the stack
                        self.e.push(LI); // Load the left integer
                    }
                    _ => {
                        panic!("{}: bad lvalue in pre-increment", self.line); // Handle bad lvalue
                    }
                }
            
                // Push the size of the type onto the stack
                self.e.push(PSH); // Push
                let size = if self.ty > PTR {
                    IMM // Immediate value
                } else {
                    IMM // Immediate value for char
                };
                self.e.push(size);
                self.e.push(if t == Token::Inc { ADD } else { SUB }); // ADD for Inc, SUB for Dec
            
                // Store the result (either as a character or an integer)
                self.e.push(if self.ty == CHAR { SC } else { SI });
            }
            
            _ => {
                eprintln!("{}: bad expression", self.line);
                std::process::exit(-1);
            }
        }
    
        // precedence climbing would go here
        let mut t = self.ty; // Store the current type
    let mut d: usize; // Placeholder for jump addresses

    while let Some(precedence) = self.tk.precedence() {
        if precedence < lev {
            break; // Exit if the current token's precedence is less than the level
        }

        match self.tk {
            Token::Assign => {
                self.next();
                if self.e.last() == Some(&LC) || self.e.last() == Some(&LI) {
                    self.e.push(PSH);
                } else {
                    eprintln!("{}: bad lvalue in assignment", self.line);
                    std::process::exit(-1);
                }
                self.expr(Token::Assign.precedence().unwrap());
                self.e.push(if t == CHAR { SC } else { SI });
            }
            Token::Cond => {
                self.next();
                self.e.push(BZ);
                d = self.e.len(); // Save the current position for the jump
                self.expr(Token::Assign.precedence().unwrap());
                if let Token::Colon = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: conditional missing colon", self.line);
                    std::process::exit(-1);
                }
                self.e.push(JMP);
                let jump_pos = self.e.len();
                self.e.push(0); // Placeholder for the jump
                self.expr(Token::Cond.precedence().unwrap());
                self.e[jump_pos - 1] = (self.e.len() + 1) as i32; // Fill in the jump address
            }
            Token::Lor => {
                self.next();
                self.e.push(BNZ);
                d = self.e.len();
                self.expr(Token::Lan.precedence().unwrap());
                self.e.push((self.e.len() + 1) as i32); // Fill in the jump address
                self.ty = INT;
            }
            Token::Lan => {
                self.next();
                self.e.push(BZ);
                d = self.e.len();
                self.expr(Token::Or.precedence().unwrap());
                self.e.push((self.e.len() + 1) as i32); // Fill in the jump address
                self.ty = INT;
            }
            Token::Or => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Xor.precedence().unwrap());
                self.e.push(OR);
                self.ty = INT;
            }
            Token::Xor => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::And.precedence().unwrap());
                self.e.push(XOR);
                self.ty = INT;
            }
            Token::And => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Eq.precedence().unwrap());
                self.e.push(AND);
                self.ty = INT;
            }
            Token::Eq => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Lt.precedence().unwrap());
                self.e.push(EQ);
                self.ty = INT;
            }
            Token::Ne => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Lt.precedence().unwrap());
                self.e.push(NE);
                self.ty = INT;
            }
            Token::Lt => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Shl.precedence().unwrap());
                self.e.push(LT);
                self.ty = INT;
            }
            Token::Gt => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Shl.precedence().unwrap());
                self.e.push(GT);
                self.ty = INT;
            }
            Token::Le => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Shl.precedence().unwrap());
                self.e.push(LE);
                self.ty = INT;
            }
            Token::Ge => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Shl.precedence().unwrap());
                self.e.push(GE);
                self.ty = INT;
            }
            Token::Shl => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Add.precedence().unwrap());
                self.e.push(SHL);
                self.ty = INT;
            }
            Token::Shr => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Add.precedence().unwrap());
                self.e.push(SHR);
                self.ty = INT;
            }
            Token::Add => {
                self.next();
                self.e .push(PSH);
                self.expr(Token::Mul.precedence().unwrap());
                if (t > PTR) {
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(std::mem::size_of::<i32>() as i32);
                    self.e.push(MUL);
                }
                self.e.push(ADD);
            }
            Token::Sub => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Mul.precedence().unwrap());
                if t > PTR && t == self.ty {
                    self.e.push(SUB);
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(std::mem::size_of::<i32>() as i32);
                    self.e.push(DIV);
                    self.ty = INT;
                } else if (self.ty > PTR) {
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(std::mem::size_of::<i32>() as i32);
                    self.e.push(MUL);
                    self.e.push(SUB);
                } else {
                    self.e.push(SUB);
                }
            }
            Token::Mul => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Inc.precedence().unwrap());
                self.e.push(MUL);
                self.ty = INT;
            }
            Token::Div => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Inc.precedence().unwrap());
                self.e.push(DIV);
                self.ty = INT;
            }
            Token::Mod => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Inc.precedence().unwrap());
                self.e.push(MOD);
                self.ty = INT;
            }
            Token::Inc | Token::Dec => {
                if self.e.last() == Some(&LC) {
                    self.e.push(PSH);
                    self.e.push(LC);
                } else if self.e.last() == Some(&LI) {
                    self.e.push(PSH);
                    self.e.push(LI);
                } else {
                    eprintln!("{}: bad lvalue in post-increment", self.line);
                    std::process::exit(-1);
                }
                self.e.push(PSH);
                self.e.push(IMM);
                self.e.push(if self.ty > PTR { std::mem::size_of::<i32>() as i32 } else { std::mem::size_of::<char>() as i32 });
                self.e.push(if self.tk == Token::Inc { ADD } else { SUB });
                self.e.push(if self.ty == CHAR { SC } else { SI });
                self.e.push(PSH);
                self.e.push(IMM);
                self.e.push(if self.ty > PTR { std::mem::size_of::<i32>() as i32 } else { std::mem::size_of::<char>() as i32 });
                self.e.push(if self.tk == Token::Inc { SUB } else { ADD });
                self.next();
            }
            Token::Brak => {
                self.next();
                self.e.push(PSH);
                self.expr(Token::Assign.precedence().unwrap());
                if let Token::RParen = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: close bracket expected", self.line);
                    std::process::exit(-1);
                }
                if t > PTR {
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(std::mem::size_of::<i32>() as i32);
                    self.e.push(MUL);
                } else if t < PTR {
                    eprintln!("{}: pointer type expected", self.line);
                    std::process::exit(-1);
                }
                self.e.push(ADD);

                self.ty = t - PTR; // Assign the new type
                self.e.push(if self.ty == CHAR { LC } else { LI });
            }
            _ => {
                eprintln!("{}: compiler error tk={:?}", self.line, self.tk);
                std::process::exit(-1);
            }
        }
    }
    
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

    fn stmt(&mut self) {
        let mut a: usize;
        let mut b: usize;
  
        match self.tk {
            Token::If => {
                self.next();
                if let Token::LParen = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: open paren expected", self.line);
                    std::process::exit(-1);
                }
                self.expr(Token::Assign.precedence().unwrap());
                if let Token::RParen = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: close paren expected", self.line);
                    std::process::exit(-1);
                }
                self.e.push(BZ);
                b = self.e.len();
                self.e.push(0);
                self.stmt();
                if let Token::Else = self.tk {
                    self.e[b] = (self.e.len() + 1) as i32;
                    self.e.push(JMP);
                    b = self.e.len();
                    self.next();
                    self.stmt();
                }
                self.e[b] = (self.e.len() + 1) as i32;
            }
            Token::While => {
                self.next();
                a = self.e.len() + 1;
                if let Token::LParen = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: open paren expected", self.line);
                    std::process::exit(-1);
                }
                self.expr(Token::Assign.precedence().unwrap());
                if let Token::RParen = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: close paren expected", self.line);
                    std::process::exit(-1);
                }
                self.e.push(BZ);
                b = self.e.len();
                self.e.push(0);
                self.stmt();
                self.e.push(JMP);
                self.e.push(a as i32);
                self.e[b] = (self.e.len() + 1) as i32;
            }
            Token::Do => {
                self.next();
                a = self.e.len() + 1; // Beginning of do-while body
                self.stmt();
                if let Token::While = self.tk {
                    self.next();
                    if let Token::LParen = self.tk {
                        self.next();
                    } else {
                        eprintln!("{}: open paren expected after while in do-while", self.line);
                        std::process::exit(-1);
                    }
                    self.expr(Token::Assign.precedence().unwrap());
                    if let Token::RParen = self.tk {
                        self.next();
                    } else {
                        eprintln!("{}: close paren expected after while in do-while", self.line);
                        std::process::exit(-1);
                    }
                    self.e.push(BNZ);
                    self.e.push(a as i32);
                    if let Token::Semicolon = self.tk {
                        self.next();
                    } else {
                        eprintln!("{}: semicolon expected after do-while", self.line);
                        std::process::exit(-1);
                    }
                } else {
                    eprintln!("{}: while expected after do statement", self.line);
                    std::process::exit(-1);
                }
            }
            Token::Return => {
                self.next();
                if self.tk != Token::Semicolon {
                    self.expr(Token::Assign.precedence().unwrap());
                }
                self.e.push(LEV);
                if let Token::Semicolon = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: semicolon expected", self.line);
                    std::process::exit(-1);
                }
            }
            Token::LBrace => {
                self.next();
                while self.tk != Token::RBrace {
                    self.stmt();
                }
                self.next();
            }
            Token::Semicolon => {
                self.next();
            }
            _ => {
                self.expr(Token::Assign.precedence().unwrap());
                if let Token::Semicolon = self.tk {
                    self.next();
                } else {
                    eprintln!("{}: semicolon expected", self.line);
                    std::process::exit(-1);
                }
            }
        }
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
