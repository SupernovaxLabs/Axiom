#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Fn,
    Let,
    Ident(String),
    Int(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Semicolon,
    Eof,
    Error(char),
}
