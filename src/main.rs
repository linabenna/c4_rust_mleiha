// Constants for tokens and opcodes- add more
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read}; // Import Read trait
use std::ptr;

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


// FROM C NEXT() TO RUST LEXER CLASS LOGIC EXPLAINED 
// in the original c4 compiler the next function used a manual character 
// pointer to iterate over the source code and classify tokens...

// since pointers are considered unsafe in rust, other data structures like 
// string slices (&str), indexes and options can track the source code.

struct Lexer<'a> {
    source: &'a str, // the full input source code to tokenize
    position: usize, // current index in the source string
    line: usize, // current line number- for debugging like c4
    current_char: Option<char>, // currently read character 
    keywords: HashMap<&'a str, Token>, // hashmap that maps strings like "if" and "return" to token types
}

// SInce structs in Rust can act like classes, we can define the 
// following constructors and methods to build this lexer

// new(): constructor to initialize the lexer - setting position, line, first char, populate keywords

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self { // constructor that initializes a new lexer instance

        // default constructor --= default values
        let mut lexer = Lexer {
            source, // full source code
            position: 0, // start reading from pos 0 = the beginning
            line: 1, // error tracking from line 1 = the first line
            current_char: None,
            keywords: HashMap::new(), // empty keyword map for now
        };

        // we need to populate the keyword map with reserved words
        Token::Else;
        lexer.keywords.insert("enum", Token::Enum);
        lexer.keywords.insert("if", Token::If);
        lexer.keywords.insert("int", Token::Int);
        lexer.keywords.insert("return", Token::Return);
        lexer.keywords.insert("sizeof", Token::Sizeof);
        lexer.keywords.insert("while", Token::While);

        // advance to the first character of the source code
        // this mimics how c4 manually reads the first char into a variable
        lexer.advance();
        lexer
    }

    // here this function moves to the next character in the source code
    fn advance(&mut self) {
        // if we're not at the end of the code, get the next byte and convert it to a char
        self.current_char = if self.position < self.source.len() {
            Some(self.source.as_bytes()[self.position] as char)
        } else { // we reached the end of the source code
            None
        };
        self.position += 1; // move the reading position to the next 
    }

    // this function allows us to look at the next character in the source code 
    // without actually advancing the current reading position (used for lookahead logic)
    fn peek(&self) -> Option<char> { // sampe implementation as advance() method 
        if self.position < self.source.len() {
            Some(self.source.as_bytes()[self.position] as char)
        } else {
            None
        }
    }

    // this function gets the next token from the source code
    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current_char { // loop while there is a current character to process
            match c {
                ' ' | '\t' | '\r' => self.advance(), // skip whitespace characters
                '\n' => { // a newline is found?
                    self.line += 1; // then increment line number 
                    self.advance();
                }

                // handle single-line comments and hash comments
                '/' => {
                    if self.peek() == Some('/') {
                        while self.current_char != Some('\n') && self.current_char.is_some() {
                            self.advance();
                        }
                    } else {
                        self.advance();
                        return Some(Token::Div);
                    }
                }
                '#' => {
                    while self.current_char != Some('\n') && self.current_char.is_some() {
                        self.advance();
                    }
                }
                // handle string literal
                '"' => {
                    self.advance();
                    let mut string = String::new();
                    while let Some(ch) = self.current_char {
                        if ch == '"' {
                            break;
                        }
                        string.push(ch);
                        self.advance();
                    }
                    self.advance(); // skip the closing "
                    return Some(Token::Str(string));
                }
                // handle character literal
                '\'' => {
                    self.advance();
                    let ch = self.current_char?;
                    self.advance();
                    self.advance(); // skip closing '
                    return Some(Token::Char(ch));
                }
                // handle operators
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') { // if it's == then the token is Eq
                        self.advance();
                        return Some(Token::Eq);
                    }
                    return Some(Token::Assign); // else its an assignment 
                }
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Some(Token::Ne);
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Some(Token::Le);
                    }
                    return Some(Token::Lt);
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Some(Token::Ge);
                    }
                    return Some(Token::Gt);
                }
                '+' => {
                    self.advance();
                    if self.current_char == Some('+') {
                        self.advance();
                        return Some(Token::Inc);
                    }
                    return Some(Token::Add);
                }
                '-' => {
                    self.advance();
                    if self.current_char == Some('-') {
                        self.advance();
                        return Some(Token::Dec);
                    }
                    return Some(Token::Sub);
                }
                '*' => {
                    self.advance();
                    return Some(Token::Mul);
                }
                '%' => {
                    self.advance();
                    return Some(Token::Mod);
                }
                '(' => {
                    self.advance();
                    return Some(Token::LParen);
                }
                ')' => {
                    self.advance();
                    return Some(Token::RParen);
                }
                '{' => {
                    self.advance();
                    return Some(Token::LBrace);
                }
                '}' => {
                    self.advance();
                    return Some(Token::RBrace);
                }
                ';' => {
                    self.advance();
                    return Some(Token::Semicolon);
                }
                ':' => {
                    self.advance();
                    return Some(Token::Colon);
                }
                ',' => {
                    self.advance();
                    return Some(Token::Comma);
                }

                '0'..='9' => return Some(self.lex_number()), // if a digit is found, parse a number token

                // if a letter or underscore is found, parse an identifier or keyword
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.lex_identifier()), 

                '"' => return Some(self.lex_string()), // detect string literals

                _ => { // if an unknown character is found, just skip it
                    self.advance(); // skip
                }
            }
        }
        None 
    }

    // parses a sequence of digits into a number token
    fn lex_number(&mut self) -> Token {
        let mut value: i32 = 0;
        while let Some(c) = self.current_char {
            // if the character is a digit, convert it to a number and build the full value
            if c.is_digit(10) {
                value = value * 10 + c.to_digit(10).unwrap() as i32;
                self.advance();
            } else {
                break; // stop reading if it's not a digit
            }
        }
        Token::Num(value) // return the number as a token
    }

    // this method parses an identifier or a reserved keyword from c
    fn lex_identifier(&mut self) -> Token {
        let start = self.position - 1; // this si the satrting pos of the identifier

        while let Some(c) = self.current_char { // keep reading letters, digits, or underscores
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier = &self.source[start..self.position - 1]; // get identifier from source code

        if let Some(keyword) = self.keywords.get(identifier) { // check if the keyword is known 
            keyword.clone() // if known, return the keyword token
        } else {
            // otherwise, return it as a regular identifier
            Token::Id(identifier.to_string())
        }
    }

    // handlling when printf is printing full string
    fn lex_string(&mut self) -> Token {
        let start = self.position; // mark the start of the string
        self.advance(); // skip the opening quote
        // Read characters until we find the closing quote or end of input
        while let Some(c) = self.current_char {
            if c == '"' {
                break; // found the closing quote
            }
            self.advance();
        }
        let string = &self.source[start..self.position - 1]; // extract the string content
        self.advance(); // skip the closing quote
        Token::Str(string.to_string()) // return the string token
    }
}
    
///////////////////////// Parser Implementation Begins ////////////////////////
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


// fn main() {
//     let mut state = Parser::new();
    
//     // Test with a number
//     state.tk = Token::Num(42);
//     state.expr(0);
//     println!("Emitted code (number): {:?}", state.e);

//     // Reset state
//     state.e.clear();

//     // Test with a string
//     state.tk = Token::Str("hello".to_string());
//     state.expr(0);
//     println!("Emitted code (string): {:?}", state.e);
//     println!("Data section (string): {:?}", state.data);
// }

const POOL_SIZE: usize = 256 * 1024; // Define POOL_SIZE

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
    let mut sp = vec![0; POOL_SIZE]; // Stack area

    // Read the source file
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();

    // Initialize the lexer
    let mut lexer = Lexer::new(&source);
    
    // Parse declarations
    let mut line = 1;
    let mut tk = lexer.next_token(); // Get the first token
    let mut ival = 0; // Define ival to hold enum initializer values
    //let mut idmain = None;
    let mut idmain: Option<Parser> = None; // Define idmain to hold the main function reference


    while tk != Some(Token::None) {
        let mut bt = INT; // Base type
        match tk {
            Some(Token::Int) => {
                tk = lexer.next_token();
            }
            Some(Token::Char(_)) => {
                tk = lexer.next_token();
                bt = CHAR;
            }
            Some(Token::Enum) => {
                tk = lexer.next_token();
                if tk == Some(Token::LBrace) {
                    tk = lexer.next_token();
                    let mut i = 0;
                    while tk != Some(Token::RBrace) {
                        if let Some(Token::Id(_)) = tk {
                            eprintln!("{}: bad enum identifier {:?}", line, tk);
                            return;
                        }
                        tk = lexer.next_token();
                        if tk == Some(Token::Assign) {
                            tk = lexer.next_token();
                            if let Some(Token::Num(_)) = tk {
                                eprintln!("{}: bad enum initializer", line);
                                return;
                            }
                            // Assuming you have a way to get the value of the number
                            // For example, if you have a method to get the value of the number token
                            ival = get_number_value(tk); // Define this function to extract the value
                            tk = lexer.next_token();
                        }
                        // Add to symbol table
                        // Assuming you have a way to add to the symbol table
                        // id[Class] = Num; id[Type] = INT; id[Val] = i++;
                        if tk == Some(Token::Comma) {
                            tk = lexer.next_token();
                        }
                    }
                    tk = lexer.next_token();
                }
            }
            _ => {
                eprintln!("{}: unexpected token {:?}", line, tk);
                return;
            }
        }

        // Handle global declarations
        while tk != Some(Token::Semicolon) && tk != Some(Token::RBrace) {
            let mut ty = bt;
            while tk == Some(Token::Mul) {
                tk = lexer.next_token();
                ty += PTR; // Assuming PTR is defined
            }
            if let Some(Token::Id(_)) = tk {
                eprintln!("{}: bad global declaration {:?}", line, tk);
                return;
            }
            // Check for duplicate definitions
            // Assuming you have a way to check for duplicates
            tk = lexer.next_token();
            // id[Type] = ty; // Assuming you have a way to set the type

            if tk == Some(Token::LParen) { // Function
                // Handle function declaration
                // Similar to the C code, handle parameters and local declarations
            } else {
                // Handle global variable declaration
                // id[Class] = Glo; id[Val] = (int)data; // Assuming you have a way to set these
                // data += std::mem::size_of::<i32>(); // Assuming you have a way to manage data
            }
            if tk == Some(Token::Comma) {
                tk = lexer.next_token();
            }
        }
        tk = lexer.next_token();
    }

    // Assuming idmain is defined and points to the main function
    if let Some(main_func) = idmain {
        // let pc = main_func.ival as *mut i32; // Get the program counter from the main function
        let mut pc = main_func.ival as *mut i32;  // Declare pc as mutable
        if src {
            return;
        }

        // Setup stack
        let mut bp = unsafe{sp.as_mut_ptr().add(POOL_SIZE)};
        let mut sp = bp;
        let mut t = 0; // Define t as needed
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
            pc = unsafe{pc.add(1)}; // Move to the next instruction
            cycle += 1;

            if debug {
                println!("{}> {:?}", cycle, i); // Print debug information
            }

            match i {
                LEA => {
                    let a = unsafe { *(bp.add((*pc).try_into().unwrap())) }; // Load local address
                    pc = unsafe{pc.add(1)};
                }
                IMM => {
                    let a = unsafe { *pc }; // Load immediate value
                    pc = unsafe{pc.add(1)};
                }
                JMP => {
                    pc = unsafe { *(pc as *mut *mut i32) }; // Jump
                }
                JSR => {
                    unsafe {
                        sp = sp.sub(1);
                        *sp = (pc as i32) + 1; // Save return address
                    }
                    pc = unsafe { *(pc as *mut *mut i32) }; // Jump to subroutine
                }
                BZ => {
                    let a = unsafe { *sp }; // Fetch value from stack
                    if a == 0 {
                        pc = unsafe { *(pc as *mut *mut i32) }; // Branch if zero
                    } else {
                        pc = unsafe{pc.add(1)};
                    }
                }
                BNZ => {
                    let a = unsafe { *sp }; // Fetch value from stack
                    if a != 0 {
                        pc = unsafe { *(pc as *mut *mut i32) }; // Branch if not zero
                    } else {
                        pc =unsafe{ pc.add(1)};
                    }
                }
                ENT => {
                    unsafe {
                        sp = sp.sub(1);
                        *sp = bp as i32; // Enter subroutine
                        sp = sp.sub((*pc as usize));  // Adjust stack pointer
                    }
                    bp = sp;
                    pc = unsafe{pc.add(1)};
                }
                ADJ => {
                    // sp = unsafe {sp.add(*pc)}; // Stack adjust
                    sp = unsafe {sp.add((*pc).try_into().unwrap())}; // Stack adjust
                    pc = unsafe {pc.add(1)};
                }
                LEV => {
                    sp = bp; // Leave subroutine
                    bp = unsafe { *sp as *mut i32 };
                    pc = unsafe { *sp.add(1) as *mut i32 };
                }
                LI => {
                    let a = unsafe { *(sp as *mut i32) }; // Load integer from stack
                }
                LC => {
                    let a = unsafe { *(sp as *mut u8) }; // Load character from stack
                }
                SI => {
                    let a = unsafe { *sp }; // Fetch integer from stack
                    unsafe {
                        *(sp as *mut i32) = a; // Store integer
                    }
                    sp = unsafe{sp.add(1)};
                }
                SC => {
                    let a = unsafe { *sp }; // Fetch character from stack
                    unsafe {
                        *(sp as *mut u8) = a as u8; // Store character
                    }
                    sp = unsafe{sp.add(1)};
                }
                PSH => {
                    let a = unsafe { *sp }; // Fetch value to push onto stack
                    unsafe {
                        sp = sp.sub(1);
                        *sp = a; // Push value
                    }
                }
                EXIT => {
                    unsafe {
                        println!("exit({}) cycle = {}", *sp, cycle);
                    }
                    return; // Exit the program
                }
                _ => {
                    eprintln!("unknown instruction = {}! cycle = {}", i, cycle);
                    return; // Handle unknown instruction
                }
            }
            
            
        }
    } else {
        eprintln!("Main function not defined.");
    }
}

// Define a function to extract the value from a number token
fn get_number_value(token: Option<Token>) -> i32 {
    match token {
        Some(Token::Num(value)) => value,
        _ => 0, // Default value if not a number
    }
}

