use crate::interp::InterpreterError;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    Text(String),
    Nil,
    Ident(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Let,
    Var,
    Fn,
    Return,
    If,
    Else,
    While,
    True,
    False,
    Nil,
    Ident(String),
    Number(f64),
    Text(String),
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AndAnd,
    OrOr,
    Semi,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eof,
}

pub fn parse_program(src: &str) -> Result<Vec<Stmt>, InterpreterError> {
    let tokens = lex(src)?;
    Parser::new(tokens).parse_program()
}

fn lex(src: &str) -> Result<Vec<Token>, InterpreterError> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch.is_whitespace() {
            i += 1;
            continue;
        }

        if ch == '/' && chars.get(i + 1) == Some(&'/') {
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        let token = match ch {
            '+' => {
                i += 1;
                Token::Plus
            }
            '-' => {
                i += 1;
                Token::Minus
            }
            '*' => {
                i += 1;
                Token::Star
            }
            '/' => {
                i += 1;
                Token::Slash
            }
            '!' => {
                i += 1;
                if chars.get(i) == Some(&'=') {
                    i += 1;
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '=' => {
                i += 1;
                if chars.get(i) == Some(&'=') {
                    i += 1;
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '>' => {
                i += 1;
                if chars.get(i) == Some(&'=') {
                    i += 1;
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '<' => {
                i += 1;
                if chars.get(i) == Some(&'=') {
                    i += 1;
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '&' => {
                i += 1;
                if chars.get(i) == Some(&'&') {
                    i += 1;
                    Token::AndAnd
                } else {
                    return Err(InterpreterError::parse("single `&` is not supported"));
                }
            }
            '|' => {
                i += 1;
                if chars.get(i) == Some(&'|') {
                    i += 1;
                    Token::OrOr
                } else {
                    return Err(InterpreterError::parse("single `|` is not supported"));
                }
            }
            ';' => {
                i += 1;
                Token::Semi
            }
            ',' => {
                i += 1;
                Token::Comma
            }
            '(' => {
                i += 1;
                Token::LParen
            }
            ')' => {
                i += 1;
                Token::RParen
            }
            '{' => {
                i += 1;
                Token::LBrace
            }
            '}' => {
                i += 1;
                Token::RBrace
            }
            '"' => {
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(InterpreterError::parse("unterminated string literal"));
                }
                let text: String = chars[start..i].iter().collect();
                i += 1;
                Token::Text(text)
            }
            c if c.is_ascii_digit() => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let value: String = chars[start..i].iter().collect();
                let number = value
                    .parse::<f64>()
                    .map_err(|_| InterpreterError::parse(format!("invalid number `{value}`")))?;
                Token::Number(number)
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();
                match ident.as_str() {
                    "let" => Token::Let,
                    "var" => Token::Var,
                    "fn" => Token::Fn,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "true" => Token::True,
                    "false" => Token::False,
                    "nil" => Token::Nil,
                    _ => Token::Ident(ident),
                }
            }
            _ => {
                return Err(InterpreterError::parse(format!(
                    "unexpected character `{ch}`"
                )));
            }
        };

        tokens.push(token);
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(mut self) -> Result<Vec<Stmt>, InterpreterError> {
        let mut statements = Vec::new();
        while !self.is_eof() {
            statements.push(self.parse_stmt()?);
        }
        Ok(statements)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, InterpreterError> {
        let stmt = if self.matches(&Token::Let) || self.matches(&Token::Var) {
            self.parse_let()?
        } else if self.matches(&Token::Fn) {
            self.parse_fn()?
        } else if self.matches(&Token::Return) {
            self.parse_return()?
        } else if self.matches(&Token::If) {
            self.parse_if()?
        } else if self.matches(&Token::While) {
            self.parse_while()?
        } else if self.matches(&Token::LBrace) {
            self.parse_block()?
        } else {
            self.parse_expr_or_assign()?
        };

        if self.matches(&Token::Semi) {
            self.pos += 1;
        }

        Ok(stmt)
    }

    fn parse_let(&mut self) -> Result<Stmt, InterpreterError> {
        let mutable = self.matches(&Token::Var);
        self.pos += 1;
        let name = self.expect_ident()?;
        self.expect(&Token::Equal)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Let {
            name,
            mutable,
            value,
        })
    }

    fn parse_fn(&mut self) -> Result<Stmt, InterpreterError> {
        self.expect(&Token::Fn)?;
        let name = self.expect_ident()?;
        self.expect(&Token::LParen)?;
        let mut params = Vec::new();
        if !self.matches(&Token::RParen) {
            loop {
                params.push(self.expect_ident()?);
                if self.matches(&Token::Comma) {
                    self.pos += 1;
                    continue;
                }
                break;
            }
        }
        self.expect(&Token::RParen)?;
        let body = match self.parse_block()? {
            Stmt::Block(stmts) => stmts,
            _ => unreachable!("parse_block always returns Stmt::Block"),
        };
        Ok(Stmt::Fn { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, InterpreterError> {
        self.expect(&Token::Return)?;
        let value = if self.matches(&Token::Semi) || self.matches(&Token::RBrace) {
            Expr::Nil
        } else {
            self.parse_expr()?
        };
        Ok(Stmt::Return(value))
    }

    fn parse_if(&mut self) -> Result<Stmt, InterpreterError> {
        self.expect(&Token::If)?;
        let condition = self.parse_expr()?;
        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.matches(&Token::Else) {
            self.pos += 1;
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, InterpreterError> {
        self.expect(&Token::While)?;
        let condition = self.parse_expr()?;
        let body = Box::new(self.parse_stmt()?);
        Ok(Stmt::While { condition, body })
    }

    fn parse_block(&mut self) -> Result<Stmt, InterpreterError> {
        self.expect(&Token::LBrace)?;
        let mut body = Vec::new();
        while !self.matches(&Token::RBrace) {
            if self.is_eof() {
                return Err(InterpreterError::parse("unterminated block"));
            }
            body.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::Block(body))
    }

    fn parse_expr_or_assign(&mut self) -> Result<Stmt, InterpreterError> {
        if let (Some(Token::Ident(name)), Some(Token::Equal)) =
            (self.tokens.get(self.pos), self.tokens.get(self.pos + 1))
        {
            let name = name.clone();
            self.pos += 2;
            let value = self.parse_expr()?;
            Ok(Stmt::Assign { name, value })
        } else {
            Ok(Stmt::Expr(self.parse_expr()?))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, InterpreterError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_and()?;
        while self.matches(&Token::OrOr) {
            self.pos += 1;
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_equality()?;
        while self.matches(&Token::AndAnd) {
            self.pos += 1;
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = if self.matches(&Token::EqualEqual) {
                BinaryOp::Eq
            } else if self.matches(&Token::BangEqual) {
                BinaryOp::Ne
            } else {
                break;
            };
            self.pos += 1;
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_add_sub()?;
        loop {
            let op = if self.matches(&Token::Greater) {
                BinaryOp::Gt
            } else if self.matches(&Token::GreaterEqual) {
                BinaryOp::Ge
            } else if self.matches(&Token::Less) {
                BinaryOp::Lt
            } else if self.matches(&Token::LessEqual) {
                BinaryOp::Le
            } else {
                break;
            };
            self.pos += 1;
            let right = self.parse_add_sub()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_add_sub(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_mul_div()?;
        loop {
            let op = if self.matches(&Token::Plus) {
                BinaryOp::Add
            } else if self.matches(&Token::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };
            self.pos += 1;
            let right = self.parse_mul_div()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, InterpreterError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = if self.matches(&Token::Star) {
                BinaryOp::Mul
            } else if self.matches(&Token::Slash) {
                BinaryOp::Div
            } else {
                break;
            };
            self.pos += 1;
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, InterpreterError> {
        if self.matches(&Token::Minus) {
            self.pos += 1;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.matches(&Token::Bang) {
            self.pos += 1;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_unary()?),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, InterpreterError> {
        match self.current().clone() {
            Token::Number(n) => {
                self.pos += 1;
                Ok(Expr::Number(n))
            }
            Token::True => {
                self.pos += 1;
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.pos += 1;
                Ok(Expr::Bool(false))
            }
            Token::Nil => {
                self.pos += 1;
                Ok(Expr::Nil)
            }
            Token::Text(s) => {
                self.pos += 1;
                Ok(Expr::Text(s))
            }
            Token::Ident(name) => {
                self.pos += 1;
                if self.matches(&Token::LParen) {
                    self.pos += 1;
                    let mut args = Vec::new();
                    if !self.matches(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.matches(&Token::Comma) {
                                self.pos += 1;
                                continue;
                            }
                            break;
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => {
                self.pos += 1;
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            token => Err(InterpreterError::parse(format!(
                "unexpected token {token:?}"
            ))),
        }
    }

    fn expect_ident(&mut self) -> Result<String, InterpreterError> {
        match self.current().clone() {
            Token::Ident(name) => {
                self.pos += 1;
                Ok(name)
            }
            _ => Err(InterpreterError::parse("expected identifier")),
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), InterpreterError> {
        if self.matches(expected) {
            self.pos += 1;
            Ok(())
        } else {
            Err(InterpreterError::parse(format!(
                "expected {expected:?}, found {:?}",
                self.current()
            )))
        }
    }

    fn matches(&self, expected: &Token) -> bool {
        use Token::*;
        matches!(
            (self.current(), expected),
            (Let, Let)
                | (Var, Var)
                | (Fn, Fn)
                | (Return, Return)
                | (If, If)
                | (Else, Else)
                | (While, While)
                | (True, True)
                | (False, False)
                | (Nil, Nil)
                | (Plus, Plus)
                | (Minus, Minus)
                | (Star, Star)
                | (Slash, Slash)
                | (Bang, Bang)
                | (BangEqual, BangEqual)
                | (Equal, Equal)
                | (EqualEqual, EqualEqual)
                | (Greater, Greater)
                | (GreaterEqual, GreaterEqual)
                | (Less, Less)
                | (LessEqual, LessEqual)
                | (AndAnd, AndAnd)
                | (OrOr, OrOr)
                | (Semi, Semi)
                | (Comma, Comma)
                | (LParen, LParen)
                | (RParen, RParen)
                | (LBrace, LBrace)
                | (RBrace, RBrace)
                | (Eof, Eof)
        )
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn is_eof(&self) -> bool {
        self.matches(&Token::Eof)
    }
}
