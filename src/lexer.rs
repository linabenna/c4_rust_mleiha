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

struct Lexer<'a> {
    source: &'a str,
    position: usize,
    line: usize,
    current_char: Option<char>,
    keywords: HashMap<&'a str, Token>,
}


// fn main() {
//     // let source_code = "int main() {  printf('hello, world\n'); }";
//     let source_code = "int main() {  return 0; }";
//     let mut lexer = Lexer::new(source_code);

//     while let Some(token) = lexer.next_token() {
//         println!("{:?}", token);
//     }
// }