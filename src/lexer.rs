use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    Num(i64),
    Id(String),
    Char(char),
    Else, Enum, If, Int, Return, Sizeof, While, Assign, Cond, Lor, Lan, Or,
    Xor, And, Eq, Ne, Lt, Gt, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec,
    Brak, LParen, RParen, LBrace, RBrace, Comma, Colon, Semicolon, RBrak,
}

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
}

// next_token(): the main function that advances through the source and yields tokens
// other helper methods like advance(), peek(), skip_whitespace(), read_number(), read_id()

// fn main() {
//     // let source_code = "int main() {  printf('hello, world\n'); }";
//     let source_code = "int main() {  return 0; }";
//     let mut lexer = Lexer::new(source_code);

//     while let Some(token) = lexer.next_token() {
//         println!("{:?}", token);
//     }
// }