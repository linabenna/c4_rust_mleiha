use std::collections::HashMap;

// we start by re-writing all variables declared in c4
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    Num(i64),
    Id(String),
    Char(char),
    Else, Enum, If, Int, Return, Sizeof, While, Assign, Cond, Lor, Lan, Or,
    Xor, And, Eq, Ne, Lt, Gt, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec,
    Brak, LParen, RParen, LBrace, RBrace, Comma, Colon, Semicolon, RBrak,
}


// these are opcode constants the vm can execute
const LEA: i64 = 0; // load effective address
const IMM: i64 = 1; // load immediate value
const JMP: i64 = 2; // unconditional jump
const JSR: i64 = 3; // jump to subroutine (function call)
const BZ: i64 = 4; // branch if zero
const BNZ: i64 = 5; // branch if not zero
const ENT: i64 = 6; // enter function (setup stack frame)
const ADJ: i64 = 7; // adjust stack
const LEV: i64 = 8; // leave function
const LI: i64 = 9; // load integer from memory
const LC: i64 = 10; // load character from memory
const SI: i64 = 11; // store integer to memory
const SC: i64 = 12; // store character to memory
const PSH: i64 = 13; // push value onto stack

// the rest below are arithmetic and logical operations
const OR: i64 = 14;
const XOR: i64 = 15;
const AND: i64 = 16;
const EQ: i64 = 17;
const NE: i64 = 18;
const LT: i64 = 19;
const GT: i64 = 20;
const LE: i64 = 21;
const GE: i64 = 22;
const SHL: i64 = 23;
const SHR: i64 = 24;
const ADD: i64 = 25;
const SUB: i64 = 26;
const MUL: i64 = 27;
const DIV: i64 = 28;
const MOD: i64 = 29;

// below are system calls
const OPEN: i64 = 30;
const READ: i64 = 31;
const CLOS: i64 = 32;
const PRTF: i64 = 33;
const MALC: i64 = 34;
const FREE: i64 = 35;
const MSET: i64 = 36;
const MCMP: i64 = 37;
const EXIT: i64 = 38;

// data types
const CHAR: i64 = 0;
const INT: i64 = 1;
const PTR: i64 = 2;

// since identifiers are stored in flat arrays (not structs), we use fixed offsets
// each identifier entry will have multiple fields like token type, type, value, etc.
const Tk: usize = 0;
const Hash: usize = 1;
const Name: usize = 2;
const Class: usize = 3;
const Type: usize = 4;
const Val: usize = 5;
const HClass: usize = 6;
const HType: usize = 7;
const HVal: usize = 8;
const Idsz: usize = 9;


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

    // this function gets the next token from the source code
    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current_char { // loop while there is a current character to process
            match c {
                ' ' | '\t' | '\r' => self.advance(), // skip whitespace characters
                '\n' => { // a newline is found?
                    self.line += 1; // then increment line number 
                    self.advance();
                }
                '0'..='9' => return Some(self.lex_number()), // if a digit is found, parse a number token

                // if a letter or underscore is found, parse an identifier or keyword
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.lex_identifier()), 

                // return simple character tokens like parentheses and semicolons directly
                '(' | ')' | '{' | '}' | ';' => {
                    let token = Token::Char(c);
                    self.advance();
                    return Some(token);
                }
                _ => { // if an unknown character is found, just skip it
                    self.advance(); // skip
                }
            }
        }
        None 
    }

    // parses a sequence of digits into a number token
    fn lex_number(&mut self) -> Token {
        let mut value = 0;
        while let Some(c) = self.current_char {
            // if the character is a digit, convert it to a number and build the full value
            if c.is_digit(10) {
                value = value * 10 + c.to_digit(10).unwrap() as i64;
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
    
}

fn main() {
    let source_code = "int main() {  printf('hello, world\n'); }";
    // let source_code = "int main() {  return 0; }";
    let mut lexer = Lexer::new(source_code);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}