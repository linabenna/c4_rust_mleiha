use c4_rust_mleiha::parser::Parser;
use c4_rust_mleiha::lexer::Lexer;

#[test]
fn test_parser_main() {
    let code = "int main() { return 1 + 2; }";
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(&mut lexer);
    
    parser.parse_program(); // Adjust based on function name
    println!("Parser ran successfully.");
}
