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

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Num(i32),
    StringLiteral(i32),
    Sizeof,
    Id(String),
    Int,
    Char,
    Mul,
    Add,
    Sub,
    Inc,
    Dec,
    Assign,
    Cond,
    Lor,
    Lan,
    Or,
    Xor,
    And,
    Eq,
    OpenParen,
    CloseParen,
    Comma,
    Eof,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Int,
    Char,
    Ptr(Box<Type>),
}

#[derive(Clone, Debug)]
enum SymbolClass {
    Num,
    Loc,
    Glo,
    Fun,
    Sys,
}

#[derive(Clone, Debug)]
struct Symbol {
    class: SymbolClass,
    value: i32,
    ty: Type,
}

struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl SymbolTable {
    fn get(&self, id: &str) -> Option<&Symbol> {
        self.table.get(id)
    }
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
    line: i32,
    symbol_table: SymbolTable,
    e: Vec<i32>,
    ty: Type,
}

impl Parser {
    fn new(tokens: Vec<Token>, symbol_table: SymbolTable) -> Self {
        Self {
            tokens,
            index: 0,
            line: 1,
            symbol_table,
            e: Vec::new(),
            ty: Type::Int,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.index).unwrap_or(&Token::Eof)
    }

    fn next(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }

    fn expect(&mut self, expected: Token) {
        if *self.current() != expected {
            panic!("{}: expected {:?}, found {:?}", self.line, expected, self.current());
        }
        self.next();
    }

    fn expr(&mut self, _lev: i32) {
        use Token::*;
        match self.current() {
            Num(n) => {
                self.e.push(IMM);
                self.e.push(*n);
                self.next();
                self.ty = Type::Int;
            }
            StringLiteral(addr) => {
                self.e.push(IMM);
                self.e.push(*addr);
                self.next();
                while let StringLiteral(_) = self.current() {
                    self.next();
                }
                self.ty = Type::Ptr(Box::new(Type::Char));
            }
            Sizeof => {
                self.next();
                self.expect(OpenParen);
                self.ty = Type::Int;
                match self.current() {
                    Int => {
                        self.next();
                    }
                    Char => {
                        self.next();
                        self.ty = Type::Char;
                    }
                    _ => panic!("{}: expected type after sizeof", self.line),
                }
                while *self.current() == Mul {
                    self.next();
                    self.ty = Type::Ptr(Box::new(self.ty.clone()));
                }
                self.expect(CloseParen);
                self.e.push(IMM);
                let size = match self.ty {
                    Type::Char => std::mem::size_of::<u8>() as i32,
                    _ => std::mem::size_of::<i32>() as i32,
                };
                self.e.push(size);
                self.ty = Type::Int;
            }
            Id(name) => {
                let id = name.clone();
                self.next();
                let sym = self.symbol_table.get(&id).unwrap_or_else(|| {
                    panic!("{}: undefined identifier {}", self.line, id);
                });

                // Function call
                if *self.current() == OpenParen {
                    self.next();
                    let mut argc = 0;
                    while *self.current() != CloseParen {
                        self.expr(0); // Use precedence level
                        self.e.push(PSH);
                        argc += 1;
                        if *self.current() == Comma {
                            self.next();
                        }
                    }
                    self.next(); // consume ')'

                    match sym.class {
                        SymbolClass::Sys => self.e.push(sym.value),
                        SymbolClass::Fun => {
                            self.e.push(JSR);
                            self.e.push(sym.value);
                        }
                        _ => panic!("{}: bad function call", self.line),
                    }

                    if argc > 0 {
                        self.e.push(ADJ);
                        self.e.push(argc);
                    }

                    self.ty = sym.ty.clone();
                } else {
                    match sym.class {
                        SymbolClass::Num => {
                            self.e.push(IMM);
                            self.e.push(sym.value);
                            self.ty = Type::Int;
                        }
                        SymbolClass::Loc => {
                            self.e.push(LEA);
                            self.e.push(sym.value);
                            self.ty = sym.ty.clone();
                            self.e.push(match sym.ty {
                                Type::Char => LC,
                                _ => LI,
                            });
                        }
                        SymbolClass::Glo => {
                            self.e.push(IMM);
                            self.e.push(sym.value);
                            self.ty = sym.ty.clone();
                            self.e.push(match sym.ty {
                                Type::Char => LC,
                                _ => LI,
                            });
                        }
                        _ => panic!("{}: undefined variable usage", self.line),
                    }
                }
            }
            _ => panic!("{}: unsupported expression token {:?}", self.line, self.current()),
        }

        fn get_precedence(&self, token: &Token) -> i32 {
            use Token::*;
            match token {
                Assign => 1,
                Lor => 2,
                Lan => 3,
                Or => 4,
                Xor => 5,
                And => 6,
                Eq => 7,
                Add | Sub => 10,
                Mul => 20,
                _ => 0,
            }
        }
    
        fn expr(&mut self, lev: i32) {
            use Token::*;
    
            // === Parse atomic part of the expression (like literals, variables, function calls) ===
            match self.current() {
                Num(n) => {
                    self.e.push(IMM);
                    self.e.push(*n);
                    self.next();
                    self.ty = Type::Int;
                }
                StringLiteral(addr) => {
                    self.e.push(IMM);
                    self.e.push(*addr);
                    self.next();
                    while let StringLiteral(_) = self.current() {
                        self.next();
                    }
                    self.ty = Type::Ptr(Box::new(Type::Char));
                }
                Sizeof => {
                    self.next();
                    self.expect(OpenParen);
                    self.ty = Type::Int;
                    match self.current() {
                        Int => { self.next(); }
                        Char => {
                            self.next();
                            self.ty = Type::Char;
                        }
                        _ => panic!("{}: expected type in sizeof", self.line),
                    }
                    while *self.current() == Mul {
                        self.next();
                        self.ty = Type::Ptr(Box::new(self.ty.clone()));
                    }
                    self.expect(CloseParen);
                    self.e.push(IMM);
                    let size = match self.ty {
                        Type::Char => 1,
                        _ => 4,
                    };
                    self.e.push(size);
                    self.ty = Type::Int;
                }
                Id(name) => {
                    let id = name.clone();
                    self.next();
                    let sym = self.symbol_table.get(&id).expect("undefined symbol");
    
                    // === Function call ===
                    if *self.current() == OpenParen {
                        self.next();
                        let mut argc = 0;
                        while *self.current() != CloseParen {
                            self.expr(0);
                            self.e.push(PSH);
                            argc += 1;
                            if *self.current() == Comma {
                                self.next();
                            }
                        }
                        self.next(); // consume ')'
    
                        match sym.class {
                            SymbolClass::Sys => self.e.push(sym.value),
                            SymbolClass::Fun => {
                                self.e.push(JSR);
                                self.e.push(sym.value);
                            }
                            _ => panic!("{}: invalid function call", self.line),
                        }
    
                        if argc > 0 {
                            self.e.push(ADJ);
                            self.e.push(argc);
                        }
                        self.ty = sym.ty.clone();
                    } else {
                        // === Variable ===
                        match sym.class {
                            SymbolClass::Num => {
                                self.e.push(IMM);
                                self.e.push(sym.value);
                                self.ty = Type::Int;
                            }
                            SymbolClass::Loc => {
                                self.e.push(LEA);
                                self.e.push(sym.value);
                                self.e.push(match sym.ty {
                                    Type::Char => LC,
                                    _ => LI,
                                });
                                self.ty = sym.ty.clone();
                            }
                            SymbolClass::Glo => {
                                self.e.push(IMM);
                                self.e.push(sym.value);
                                self.e.push(match sym.ty {
                                    Type::Char => LC,
                                    _ => LI,
                                });
                                self.ty = sym.ty.clone();
                            }
                            _ => panic!("invalid symbol class"),
                        }
                    }
                }
                OpenParen => {
                    self.next();
                    self.expr(0);
                    self.expect(CloseParen);
                }
                Mul | And | Sub | Inc | Dec => {
                    let op = self.current().clone();
                    self.next();
                    self.expr(20); // highest precedence
    
                    match op {
                        Mul => {
                            if let Type::Ptr(_) = self.ty {
                                self.e.push(LI);
                            } else {
                                self.e.push(LC);
                            }
                        }
                        Sub => {
                            self.e.push(PUSH);
                            self.e.push(IMM);
                            self.e.push(-1);
                            self.e.push(MUL);
                        }
                        Inc | Dec => {
                            self.e.push(PUSH);
                            self.e.push(IMM);
                            self.e.push(1);
                            self.e.push(if op == Inc { ADD } else { SUB });
                        }
                        _ => {}
                    }
                }
                _ => panic!("{}: unexpected token {:?}", self.line, self.current()),
            }
    
            // === Parse binary operators ===
            loop {
                let tok_prec = self.get_precedence(self.current());
                if tok_prec < lev {
                    break;
                }
    
                let op = self.current().clone();
                self.next();
    
                if op == Assign {
                    self.e.push(PUSH);
                    self.expr(tok_prec);
                    self.e.push(STO);
                    continue;
                }
    
                self.e.push(PUSH);
                self.expr(tok_prec + 1);
    
                self.e.push(match op {
                    Add => ADD,
                    Sub => SUB,
                    Mul => MUL,
                    Eq => EQ,
                    And => AND,
                    Or => OR,
                    Xor => XOR,
                    Lor => OR,
                    Lan => AND,
                    _ => panic!("{}: unsupported operator {:?}", self.line, op),
                });
            }
        }

    }
}
