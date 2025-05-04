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

fn main() {
    // pass 
}

