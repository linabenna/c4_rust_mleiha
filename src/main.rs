use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    Num(i64),
    Id(String),
    Char(char),
    Str(String),
    Else, Enum, If, Int, Return, Sizeof, While, Assign, Cond, Lor, Lan, Or,
    Xor, And, Eq, Ne, Lt, Le, Gt, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec,
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
                    if self.current_char == Some('=') {
                        self.advance();
                        return Some(Token::Eq);
                    }
                    return Some(Token::Assign);
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

                '0'..='9' => return Some(self.lex_number()), // if a digit is found, parse a number token

                // if a letter or underscore is found, parse an identifier or keyword
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.lex_identifier()), 

                '"' => return Some(self.lex_string()), // detect string literals

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

fn main() {
    // let source_code = "int main() {  5 + 2; }";
    let source_code = r#"int main() {  printf("hello, world\n") ; }"#;
    // let source_code = "int main() {  return 0; }";
    let mut lexer = Lexer::new(source_code);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}