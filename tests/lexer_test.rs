// tests/lexer_test.rs
// pull request demo
use c4_rust_mleiha::lexer::{Lexer, Token}; // Adjust crate name if different

// Helper function to collect all tokens including EOF
fn collect_tokens(code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Some(token) => tokens.push(token),
            None => { // End of input stream from lexer
                tokens.push(Token::Eof); // Add Eof marker for tests
                break;
            }
        }
        // Safeguard against infinite loops during development
        if tokens.len() > 500 { // Adjust limit
             panic!("Lexer test safety break: Too many tokens generated for code: {}", code);
        }
    }
    tokens
}

// --- Test Cases ---

#[test]
fn integration_lexer_basic_int_assignment() {
    let code = "int x = 5;";
    let expected = vec![
        Token::Int,
        Token::Id("x".to_string()),
        Token::Assign, // Use correct token
        Token::Num(5),
        Token::Semicolon, // Use correct token
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Basic assignment");
}

#[test]
fn integration_lexer_keywords() {
    let code = "if while return else sizeof char"; // Added char
     let expected = vec![
        Token::If,
        Token::While,
        Token::Return,
        Token::Else,
        Token::Sizeof,
        Token::Char, // Added expectation
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Keywords");
}


#[test]
fn integration_lexer_simple_function() {
    let code = "int main() { return 0; }";
    let expected = vec![
        Token::Int,
        Token::Id("main".to_string()),
        Token::LParen,   // Use correct token
        Token::RParen,   // Use correct token
        Token::LBrace,   // Use correct token
        Token::Return,
        Token::Num(0),
        Token::Semicolon, // Use correct token
        Token::RBrace,   // Use correct token
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Simple function");
}


#[test]
fn integration_lexer_operators() {
    // Added more operators based on C4 enum
    let code = "+ - * / % = == != < > <= >= && || ! & | ^ << >> ++ -- ? :";
    let expected = vec![
        Token::Add, Token::Sub, Token::Mul, Token::Div, Token::Mod,
        Token::Assign, Token::Eq, Token::Ne, Token::Lt, Token::Gt, Token::Le, Token::Ge,
        Token::Lan, Token::Lor, Token::Not, Token::And, Token::Or, Token::Xor,
        Token::Shl, Token::Shr, Token::Inc, Token::Dec,
        Token::Cond, Token::Colon, // Added Cond/Colon
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Operators");
}

#[test]
fn integration_lexer_whitespace_and_newlines() {
    let code = "int\nx\n=\n5 ;";
     let expected = vec![
        Token::Int,
        Token::Id("x".to_string()),
        Token::Assign,
        Token::Num(5),
        Token::Semicolon,
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Whitespace and newlines");
}


#[test]
fn integration_lexer_comments() {
    let code = "// This is a comment\nint x; // Another comment";
    let expected = vec![
        Token::Int,
        Token::Id("x".to_string()),
        Token::Semicolon,
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Comments");
}

#[test]
fn integration_lexer_illegal_character() {
    let code = "int @ y;";
     let expected = vec![
        Token::Int,
        Token::Illegal("@".to_string()), // Expecting '@' to be treated as illegal
        Token::Id("y".to_string()),
        Token::Semicolon,
        Token::Eof,
    ];
     let actual = collect_tokens(code);
     // Compare directly now that Illegal contains the string
     assert_eq!(actual, expected, "Integration test failed: Illegal character");
}

// Add more tests for strings, chars, edge cases etc.
#[test]
fn integration_lexer_char_literal() {
    let code = "'a'";
    let expected = vec![Token::CharLit('a'), Token::Eof];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Char literal");
}

#[test]
fn integration_lexer_string_literal() {
    let code = "\"hello\"";
    let expected = vec![Token::StrLit("hello".to_string()), Token::Eof];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: String literal");
}

#[test]
fn integration_lexer_empty_input() {
    let code = "";
    let expected = vec![Token::Eof]; // Should just produce EOF
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Empty input");
}