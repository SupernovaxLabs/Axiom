use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenizes_function_signature() {
    let src = "fn main() -> i32 { let x = 42; }";
    let tokens = Lexer::new(src).tokenize();

    assert!(matches!(tokens[0].kind, TokenKind::Fn));
    assert!(matches!(tokens[1].kind, TokenKind::Ident(ref s) if s == "main"));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Arrow)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Int(ref s) if s == "42")));
    assert!(matches!(tokens.last().map(|t| &t.kind), Some(TokenKind::Eof)));
}
