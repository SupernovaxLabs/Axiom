use crate::lexer::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<char>,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars().collect(),
            index: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }

            let start = self.index;
            let kind = match ch {
                '(' => {
                    self.bump();
                    TokenKind::LParen
                }
                ')' => {
                    self.bump();
                    TokenKind::RParen
                }
                '{' => {
                    self.bump();
                    TokenKind::LBrace
                }
                '}' => {
                    self.bump();
                    TokenKind::RBrace
                }
                ',' => {
                    self.bump();
                    TokenKind::Comma
                }
                '+' => {
                    self.bump();
                    TokenKind::Plus
                }
                '*' => {
                    self.bump();
                    TokenKind::Star
                }
                '/' => {
                    self.bump();
                    TokenKind::Slash
                }
                ';' => {
                    self.bump();
                    TokenKind::Semicolon
                }
                '=' => {
                    self.bump();
                    TokenKind::Equal
                }
                '-' => {
                    self.bump();
                    if self.peek() == Some('>') {
                        self.bump();
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                c if c.is_ascii_alphabetic() || c == '_' => self.lex_ident_or_keyword(),
                c if c.is_ascii_digit() => self.lex_integer(),
                c => {
                    self.bump();
                    TokenKind::Error(c)
                }
            };

            tokens.push(Token {
                kind,
                span: Span {
                    start,
                    end: self.index,
                },
            });
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: self.src.len(),
                end: self.src.len(),
            },
        });

        tokens
    }

    fn lex_ident_or_keyword(&mut self) -> TokenKind {
        let start = self.index;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.bump();
            } else {
                break;
            }
        }
        let ident = &self.src[start..self.index];
        match ident {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            _ => TokenKind::Ident(ident.to_string()),
        }
    }

    fn lex_integer(&mut self) -> TokenKind {
        let start = self.index;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }
        TokenKind::Int(self.src[start..self.index].to_string())
    }

    fn bump(&mut self) -> Option<char> {
        if self.index >= self.chars.len() {
            return None;
        }
        let ch = self.chars[self.index];
        self.index += 1;
        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }
}
