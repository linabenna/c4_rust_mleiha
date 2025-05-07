// tests/lexer_test.rs

use c4_rust_mleiha::lexer::Lexer; // Import the Lexer from your crate

// Helper function to collect all tokens from the input code, including EOF
fn collect_tokens(code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(code); // Initialize the lexer with input code
    let mut tokens = Vec::new();      // Vector to hold tokens

    loop {
        match lexer.next_token() {
            Some(token) => tokens.push(token), // Append valid token to list
            None => {                          // If no token returned (input exhausted)
                tokens.push(Token::Eof);       // Add EOF marker explicitly
                break;
            }
        }

        // Safety check to avoid infinite loops (in case of lexer bugs)
        if tokens.len() > 500 {
            panic!(
                "Lexer test safety break: Too many tokens generated for code: {}",
                code
            );
        }
    }

    tokens // Return collected tokens
}

// --- Test Cases ---

#[test]
fn integration_lexer_basic_int_assignment() {
    // Test basic variable declaration and assignment
    let code = "int x = 5;";
    let expected = vec![
        Token::Int,                       // 'int' keyword
        Token::Id("x".to_string()),       // identifier x
        Token::Assign,                    // '='
        Token::Num(5),                    // literal 5
        Token::Semicolon,                 // ';'
        Token::Eof,                       // End of file/input
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Basic assignment");
}

#[test]
fn integration_lexer_keywords() {
    // Test multiple keywords in a row
    let code = "if while return else sizeof char";
    let expected = vec![
        Token::If, Token::While, Token::Return, Token::Else,
        Token::Sizeof, Token::Char,             // Added 'char' keyword
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Keywords");
}

#[test]
fn integration_lexer_simple_function() {
    // Test parsing a simple function declaration
    let code = "int main() { return 0; }";
    let expected = vec![
        Token::Int,
        Token::Id("main".to_string()),
        Token::LParen,   // '('
        Token::RParen,   // ')'
        Token::LBrace,   // '{'
        Token::Return,
        Token::Num(0),
        Token::Semicolon,
        Token::RBrace,   // '}'
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Simple function");
}

#[test]
fn integration_lexer_operators() {
    // Test common operators and ensure correct token mapping
    let code = "+ - * / % = == != < > <= >= && || ! & | ^ << >> ++ -- ? :";
    let expected = vec![
        Token::Add, Token::Sub, Token::Mul, Token::Div, Token::Mod,
        Token::Assign, Token::Eq, Token::Ne, Token::Lt, Token::Gt, Token::Le, Token::Ge,
        Token::Lan, Token::Lor, Token::Not, Token::And, Token::Or, Token::Xor,
        Token::Shl, Token::Shr, Token::Inc, Token::Dec,
        Token::Cond, Token::Colon,
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Operators");
}

#[test]
fn integration_lexer_whitespace_and_newlines() {
    // Test lexer’s ability to handle whitespace and newline-separated tokens
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
    // Ensure single-line comments are ignored
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
    // Test lexer’s response to an illegal character
    let code = "int @ y;";
    let expected = vec![
        Token::Int,
        Token::Illegal("@".to_string()), // '@' should be flagged as illegal
        Token::Id("y".to_string()),
        Token::Semicolon,
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Illegal character");
}

#[test]
fn integration_lexer_char_literal() {
    // Test for valid character literal
    let code = "'a'";
    let expected = vec![
        Token::CharLit('a'), // Should produce CharLit token
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Char literal");
}

#[test]
fn integration_lexer_string_literal() {
    // Test for valid string literal
    let code = "\"hello\"";
    let expected = vec![
        Token::StrLit("hello".to_string()), // Should produce StrLit token
        Token::Eof,
    ];
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: String literal");
}

#[test]
fn integration_lexer_empty_input() {
    // Test lexer on empty input
    let code = "";
    let expected = vec![Token::Eof]; // Should only return EOF
    let actual = collect_tokens(code);
    assert_eq!(actual, expected, "Integration test failed: Empty input");
}
