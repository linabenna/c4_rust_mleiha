use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Token {
    Num(i64),
    Id(String),
    Char(char),
    Else,
    Enum,
    If,
    Int,
    Return,
    Sizeof,
    While,
    Assign,
    Cond,
    Lor,
    Lan,
    Or,
    Xor,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Brak,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    RBrak,
    // Add other tokens as needed
}

// Opcodes
const LEA: i64 = 0;
const IMM: i64 = 1;
const JMP: i64 = 2;
const JSR: i64 = 3;
const BZ: i64 = 4;
const BNZ: i64 = 5;
const ENT: i64 = 6;
const ADJ: i64 = 7;
const LEV: i64 = 8;
const LI: i64 = 9;
const LC: i64 = 10;
const SI: i64 = 11;
const SC: i64 = 12;
const PSH: i64 = 13;
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
const OPEN: i64 = 30;
const READ: i64 = 31;
const CLOS: i64 = 32;
const PRTF: i64 = 33;
const MALC: i64 = 34;
const FREE: i64 = 35;
const MSET: i64 = 36;
const MCMP: i64 = 37;
const EXIT: i64 = 38;

// Types
const CHAR: i64 = 0;
const INT: i64 = 1;
const PTR: i64 = 2;

// Identifier offsets
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

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
    line: usize,
    e: Vec<i64>,
    data: Vec<u8>,
    sym: HashMap<String, Vec<i64>>,
    ty: i64,
    loc: i64,
    src: bool,
    debug: bool,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            position: 0,
            line: 1,
            e: Vec::new(),
            data: Vec::new(),
            sym: HashMap::new(),
            ty: 0,
            loc: 0,
            src: false,
            debug: false,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            self.tokens.get(self.position - 1)
        } else {
            None
        }
    }

    fn expr(&mut self, lev: i64) {
        let mut t;
        let mut d;

        if let Some(token) = self.next() {
            match token {
                Token::Num(ival) => {
                    self.e.push(IMM);
                    self.e.push(*ival);
                    self.ty = INT;
                }
                Token::Char(_) => {
                    self.e.push(IMM);
                    self.e.push(*ival);
                    while let Some(Token::Char(_)) = self.next() {}
                    self.data.push(0);
                    self.ty = PTR;
                }
                Token::Sizeof => {
                    if let Some(Token::LParen) = self.next() {
                        self.ty = INT;
                        if let Some(Token::Int) = self.next() {
                            self.ty = INT;
                        } else if let Some(Token::Char) = self.next() {
                            self.ty = CHAR;
                        }
                        while let Some(Token::Mul) = self.next() {
                            self.ty += PTR;
                        }
                        if let Some(Token::RParen) = self.next() {
                            self.e.push(IMM);
                            self.e.push(if self.ty == CHAR { std::mem::size_of::<char>() as i64 } else { std::mem::size_of::<i64>() as i64 });
                            self.ty = INT;
                        }
                    }
                }
                Token::Id(id) => {
                    d = self.sym.get(id).unwrap();
                    if let Some(Token::LParen) = self.next() {
                        t = 0;
                        while let Some(token) = self.next() {
                            if token == &Token::RParen {
                                break;
                            }
                            self.expr(Assign);
                            self.e.push(PSH);
                            t += 1;
                            if let Some(Token::Comma) = self.next() {}
                        }
                        if d[Class] == Sys {
                            self.e.push(d[Val]);
                        } else if d[Class] == Fun {
                            self.e.push(JSR);
                            self.e.push(d[Val]);
                        }
                        if t > 0 {
                            self.e.push(ADJ);
                            self.e.push(t);
                        }
                        self.ty = d[Type];
                    } else if d[Class] == Num {
                        self.e.push(IMM);
                        self.e.push(d[Val]);
                        self.ty = INT;
                    } else {
                        if d[Class] == Loc {
                            self.e.push(LEA);
                            self.e.push(self.loc - d[Val]);
                        } else if d[Class] == Glo {
                            self.e.push(IMM);
                            self.e.push(d[Val]);
                        }
                        self.e.push(if self.ty == CHAR { LC } else { LI });
                    }
                }
                Token::LParen => {
                    if let Some(Token::Int) | Some(Token::Char) = self.next() {
                        t = if let Some(Token::Int) = self.next() { INT } else { CHAR };
                        while let Some(Token::Mul) = self.next() {
                            t += PTR;
                        }
                        if let Some(Token::RParen) = self.next() {
                            self.expr(Inc);
                            self.ty = t;
                        }
                    } else {
                        self.expr(Assign);
                        if let Some(Token::RParen) = self.next() {}
                    }
                }
                Token::Mul => {
                    self.expr(Inc);
                    if self.ty > INT {
                        self.ty -= PTR;
                    }
                    self.e.push(if self.ty == CHAR { LC } else { LI });
                }
                Token::And => {
                    self.expr(Inc);
                    if self.e.last() == Some(&LC) || self.e.last() == Some(&LI) {
                        self.e.pop();
                    }
                    self.ty += PTR;
                }
                Token::Not => {
                    self.expr(Inc);
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(0);
                    self.e.push(EQ);
                    self.ty = INT;
                }
                Token::Tilde => {
                    self.expr(Inc);
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(-1);
                    self.e.push(XOR);
                    self.ty = INT;
                }
                Token::Add => {
                    self.expr(Inc);
                    self.ty = INT;
                }
                Token::Sub => {
                    self.e.push(IMM);
                    if let Some(Token::Num(ival)) = self.next() {
                        self.e.push(-ival);
                    } else {
                        self.e.push(-1);
                        self.e.push(PSH);
                        self.expr(Inc);
                        self.e.push(MUL);
                    }
                    self.ty = INT;
                }
                Token::Inc | Token::Dec => {
                    t = if let Some(Token::Inc) = self.next() { Inc } else { Dec };
                    self.expr(Inc);
                    if self.e.last() == Some(&LC) {
                        self.e.pop();
                        self.e.push(PSH);
                        self.e.push(LC);
                    } else if self.e.last() == Some(&LI) {
                        self.e.pop();
                        self.e.push(PSH);
                        self.e.push(LI);
                    }
                    self.e.push(if t == Inc { ADD } else { SUB });
                    self.e.push(if self.ty == CHAR { SC } else { SI });
                    self.e.push(PSH);
                    self.e.push(IMM);
                    self.e.push(if self.ty > PTR { std::mem::size_of::<i64>() as i64 } else { std::mem::size_of::<char>() as i64 });
                    self.e.push(if t == Inc { SUB } else { ADD });
                }
                _ => {
                    println!("{}: bad expression", self.line);
                    std::process::exit(-1);
                }
            }
        }

        while let Some(token) = self.next() {
            if token >= &lev {
                t = self.ty;
                match token {
                    Token::Assign => {
                        if self.e.last() == Some(&LC) || self.e.last() == Some(&LI) {
                            self.e.pop();
                            self.e.push(PSH);
                        }
                        self.expr(Assign);
                        self.e.push(if self.ty == CHAR { SC } else { SI });
                    }
                    Token::Cond => {
                        self.e.push(BZ);
                        d = self.e.len();
                        self.expr(Assign);
                        if let Some(Token::Colon) = self.next() {}
                        self.e[d] = self.e.len() as i64 + 3;
                        self.e.push(JMP);
                        d = self.e.len();
                        self.expr(Cond);
                        self.e[d] = self.e.len() as i64 + 1;
                    }
                    Token::Lor => {
                        self.e.push(BNZ);
                        d = self.e.len();
                        self.expr(Lan);
                        self.e[d] = self.e.len() as i64 + 1;
                        self.ty = INT;
                    }
                    Token::Lan => {
                        self.e.push(BZ);
                        d = self.e.len();
                        self.expr(Or);
                        self.e[d] = self.e.len() as i64 + 1;
                        self.ty = INT;
                    }
                    Token::Or => {
                        self.e.push(PSH);
                        self.expr(Xor);
                        self.e.push(OR);
                        self.ty = INT;
                    }
                    Token::Xor => {
                        self.e.push(PSH);
                        self.expr(And);
                        self.e.push(XOR);
                        self.ty = INT;
                    }
                    Token::And => {
                        self.e.push(PSH);
                        self.expr(Eq);
                        self.e.push(AND);
                        self.ty = INT;
                    }
                    Token::Eq => {
                        self.e.push(PSH);
                        self.expr(Lt);
                        self.e.push(EQ);
                        self.ty = INT;
                    }
                    Token::Ne => {
                        self.e.push(PSH);
                        self.expr(Lt);
                        self.e.push(NE);
                        self.ty = INT;
                    }
                    Token::Lt => {
                        self.e.push(PSH);
                        self.expr(Shl);
                        self.e.push(LT);
                        self.ty = INT;
                    }
                    Token::Gt => {
                        self.e.push(PSH);
                        self.expr(Shl);
                        self.e.push(GT);
                        self.ty = INT;
                    }
                    Token::Le => {
                        self.e.push(PSH);
                        self.expr(Shl);
                        self.e.push(LE);
                        self.ty = INT;
                    }
                    Token::Ge => {
                        self.e.push(PSH);
                        self.expr(Shl);
                        self.e.push(GE);
                        self.ty = INT;
                    }
                    Token::Shl => {
                        self.e.push(PSH);
                        self.expr(Add);
                        self.e.push(SHL);
                        self.ty = INT;
                    }
                    Token::Shr => {
                        self.e.push(PSH);
                        self.expr(Add);
                        self.e.push(SHR);
                        self.ty = INT;
                    }
                    Token::Add => {
                        self.e.push(PSH);
                        self.expr(Mul);
                        if self.ty > PTR {
                            self.e.push(PSH);
                            self.e.push(IMM);
                            self.e.push(std::mem::size_of::<i64>() as i64);
                            self.e.push(MUL);
                        }
                        self.e.push(ADD);
                    }
                    Token::Sub => {
                        self.e.push(PSH);
                        self.expr(Mul);
                        if self.ty > PTR && self.ty == t {
                            self.e.push(SUB);
                            self.e.push(PSH);
                            self.e.push(IMM);
                            self.e.push(std::mem::size_of::<i64>() as i64);
                            self.e.push(DIV);
                            self.ty = INT;
                        } else if self.ty > PTR {
                            self.e.push(PSH);
                            self.e.push(IMM);
                            self.e.push(std::mem::size_of::<i64>() as i64);
                            self.e.push(MUL);
                            self.e.push(SUB);
                        } else {
                            self.e.push(SUB);
                        }
                    }
                    Token::Mul => {
                        self.e.push(PSH);
                        self.expr(Inc);
                        self.e.push(MUL);
                        self.ty = INT;
                    }
                    Token::Div => {
                        self.e.push(PSH);
                        self.expr(Inc);
                        self.e.push(DIV);
                        self.ty = INT;
                    }
                    Token::Mod => {
                        self.e.push(PSH);
                        self.expr(Inc);
                        self.e.push(MOD);
                        self.ty = INT;
                    }
                    Token::Inc | Token::Dec => {
                        if self.e.last() == Some(&LC) {
                            self.e.pop();
                            self.e.push(PSH);
                            self.e.push(LC);
                        } else if self.e.last() == Some(&LI) {
                            self.e.pop();
                            self.e.push(PSH);
                            self.e.push(LI);
                        }
                        self.e.push(PSH);
                        self.e.push(IMM);
                        self.e.push(if self.ty > PTR { std::mem::size_of::<i64>() as i64 } else { std::mem::size_of::<char>() as i64 });
                        self.e.push(if token == &Token::Inc { ADD } else { SUB });
                        self.e.push(if self.ty == CHAR { SC } else { SI });
                        self.e.push(PSH);
                        self.e.push(IMM);
                        self.e.push(if self.ty > PTR { std::mem::size_of::<i64>() as i64 } else { std::mem::size_of::<char>() as i64 });
                        self.e.push(if token == &Token::Inc { SUB } else { ADD });
                    }
                    Token::Brak => {
                        self.e.push(PSH);
                        self.expr(Assign);
                        if let Some(Token::RBrak) = self.next() {
                            if self.ty > PTR {
                                self.e.push(PSH);
                                self.e.push(IMM);
                                self.e.push(std::mem::size_of::<i64>() as i64);
                                self.e.push(MUL);
                            } else if self.ty < PTR {
                                println!("{}: pointer type expected", self.line);
                                std::process::exit(-1);
                            }
                            self.e.push(ADD);
                            self.e.push(if self.ty == CHAR { LC } else { LI });
                        }
                    }
                    _ => {
                        println!("{}: compiler error tk={:?}", self.line, token);
                        std::process::exit(-1);
                    }
                }
            }
        }
    }
}

fn main() {
    // Example usage
    let tokens = vec![
        Token::Int, Token::Id("main".to_string()), Token::LParen, Token::RParen,
        Token::LBrace, Token::Return, Token::Num(0), Token::Semicolon, Token::RBrace,
    ];
    let mut parser = Parser::new(&tokens);
    parser.expr(0);
    println!("{:?}", parser.e);
}
